use slint::{Image, Rgba8Pixel, SharedPixelBuffer};
use std::{cell::RefCell, collections::HashMap, path::Path};

thread_local! {
    static ICON_CACHE: RefCell<HashMap<String, Image>> = RefCell::new(HashMap::new());
}

pub fn icon_for_file(path: &Path, is_directory: bool) -> Image {
    let key = path.to_string_lossy().to_string();
    ICON_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(icon) = cache.get(&key) {
            return icon.clone();
        }

        let icon = icon_for_path(path, is_directory).unwrap_or_else(default_icon);
        cache.insert(key, icon.clone());
        icon
    })
}

#[cfg(target_os = "windows")]
fn icon_for_path(path: &Path, is_directory: bool) -> Option<Image> {
    use std::os::windows::ffi::OsStrExt;
    use windows::core::PCWSTR;
    use windows::Win32::Storage::FileSystem::{FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_NORMAL};
    use windows::Win32::UI::Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_SMALLICON, SHGFI_USEFILEATTRIBUTES};

    let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    wide.push(0);
    let attrs = if is_directory {
        FILE_ATTRIBUTE_DIRECTORY
    } else {
        FILE_ATTRIBUTE_NORMAL
    };

    let mut shfile_info = SHFILEINFOW::default();
    let flags = SHGFI_ICON | SHGFI_SMALLICON | SHGFI_USEFILEATTRIBUTES;

    unsafe {
        if SHGetFileInfoW(
            PCWSTR(wide.as_ptr()),
            attrs,
            Some(&mut shfile_info as *mut _),
            std::mem::size_of::<SHFILEINFOW>() as u32,
            flags,
        ) == 0
        {
            return None;
        }

        let icon = shfile_info.hIcon;
        if icon.0 == 0 {
            return None;
        }

        let image = icon_to_image(icon);
        windows::Win32::UI::WindowsAndMessaging::DestroyIcon(icon);
        image
    }
}

#[cfg(target_os = "windows")]
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn icon_to_image(icon: windows::Win32::UI::WindowsAndMessaging::HICON) -> Option<Image> {
    use std::ffi::c_void;
    use std::mem::{size_of, zeroed};
    use windows::Win32::Graphics::Gdi::{
        BITMAP, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, CreateCompatibleDC, CreateDIBSection,
        DeleteDC, DeleteObject, GetObjectW, SelectObject, DIB_RGB_COLORS, HBRUSH, HGDIOBJ,
    };
    use windows::Win32::UI::WindowsAndMessaging::{DI_NORMAL, DrawIconEx, GetIconInfo, ICONINFO};

    let mut icon_info = ICONINFO::default();
    unsafe {
        if !GetIconInfo(icon, &mut icon_info).as_bool() {
            return None;
        }
    }

    let bitmap = if icon_info.hbmColor.0 != 0 {
        icon_info.hbmColor
    } else {
        icon_info.hbmMask
    };

    let mut bitmap_object: BITMAP = unsafe { zeroed() };
    let get_object_result = unsafe {
        GetObjectW(bitmap, size_of::<BITMAP>() as i32, Some(&mut bitmap_object as *mut _ as *mut _))
    };
    if get_object_result == 0 {
        unsafe { cleanup_icon_info(&icon_info) };
        return None;
    }

    let width = bitmap_object.bmWidth;
    let height = if icon_info.hbmColor.0 != 0 {
        bitmap_object.bmHeight
    } else {
        bitmap_object.bmHeight / 2
    };

    if width <= 0 || height <= 0 {
        cleanup_icon_info(&icon_info);
        return None;
    }

    let hdc = CreateCompatibleDC(windows::Win32::Graphics::Gdi::HDC(0));
    if hdc.0 == 0 {
        cleanup_icon_info(&icon_info);
        return None;
    }

    let bmi = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            biHeight: -height,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0 as u32,
            ..Default::default()
        },
        bmiColors: [windows::Win32::Graphics::Gdi::RGBQUAD::default()],
    };

    let mut bits: *mut c_void = std::ptr::null_mut();
    let hbitmap = CreateDIBSection(hdc, &bmi, DIB_RGB_COLORS, &mut bits, None, 0).ok()?;
    if bits.is_null() {
        DeleteDC(hdc);
        cleanup_icon_info(&icon_info);
        return None;
    }

    let old_obj = unsafe { SelectObject(hdc, HGDIOBJ(hbitmap.0)) };
    if old_obj.0 == 0 {
        unsafe { DeleteDC(hdc) };
        unsafe { cleanup_icon_info(&icon_info) };
        return None;
    }
    unsafe { DrawIconEx(hdc, 0, 0, icon, width, height, 0, HBRUSH(0), DI_NORMAL) };

    let bytes = unsafe { std::slice::from_raw_parts(bits as *const u8, (width as usize) * (height as usize) * 4) };
    let mut image_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(width as u32, height as u32);
    let pixels = image_buffer.make_mut_slice();
    for idx in 0..(width as usize * height as usize) {
        let base = idx * 4;
        pixels[idx] = Rgba8Pixel {
            r: bytes[base + 2],
            g: bytes[base + 1],
            b: bytes[base],
            a: bytes[base + 3],
        };
    }

    let _ = SelectObject(hdc, old_obj);
    let _ = DeleteObject(HGDIOBJ(hbitmap.0));
    DeleteDC(hdc);
    cleanup_icon_info(&icon_info);

    Some(Image::from_rgba8(image_buffer))
}

#[cfg(target_os = "windows")]
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn cleanup_icon_info(icon_info: &windows::Win32::UI::WindowsAndMessaging::ICONINFO) {
    use windows::Win32::Graphics::Gdi::{DeleteObject, HGDIOBJ};
    let _ = DeleteObject(HGDIOBJ(icon_info.hbmColor.0));
    let _ = DeleteObject(HGDIOBJ(icon_info.hbmMask.0));
}

#[cfg(not(target_os = "windows"))]
fn icon_for_path(_path: &Path, _is_directory: bool) -> Option<Image> {
    None
}

fn default_icon() -> Image {
    let size = 32;
    let mut image_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(size, size);
    let pixels = image_buffer.make_mut_slice();
    for pixel in pixels.iter_mut() {
        *pixel = Rgba8Pixel { r: 200, g: 200, b: 200, a: 255 };
    }
    Image::from_rgba8(image_buffer)
}

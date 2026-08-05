use crate::models::file_info::FileInfo;
use crate::platform::icon::icon_for_file;
use crate::ui_bridge::file_item::FileItem;
use crate::models::file_type::FileType;
use chrono::{DateTime, Local};


fn format_size(bytes: u64) -> String {

    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}


fn format_modified(
    time: Option<std::time::SystemTime>,
) -> String {

    match time {
        Some(t) => {
            let datetime: DateTime<Local> = t.into();

            datetime
                .format("%d/%m/%Y %H:%M")
                .to_string()
        }

        None => String::from("-"),
    }
}


fn file_type_label(file: &FileInfo) -> String {
    if file.file_type == FileType::Directory {
        "Folder".to_string()
    } else if let Some(extension) = file.extension.as_ref() {
        match extension.to_lowercase().as_str() {
            "lnk" => "Shortcut".to_string(),
            "exe" => "Application".to_string(),
            "url" => "Link".to_string(),
            "msi" => "Installer".to_string(),
            other => other.to_uppercase(),
        }
    } else {
        "File".to_string()
    }
}

pub fn map_file(
    file: &FileInfo,
) -> FileItem {
    FileItem {
        name: file.name.clone(),
        path: file.path
            .to_string_lossy()
            .to_string(),
        file_type: file_type_label(file),
        size: format_size(file.size),
        modified: format_modified(file.modified),
        icon: icon_for_file(&file.path, file.file_type == FileType::Directory),
        is_directory: file.file_type == FileType::Directory,
    }
}

pub fn map_files(
    files: &[FileInfo],
) -> Vec<FileItem> {
    files
        .iter()
        .map(map_file)
        .collect()
}

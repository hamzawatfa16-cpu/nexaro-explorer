use crate::models::file_info::FileInfo;
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


fn get_icon(
    file: &FileInfo,
) -> String {

    match file.file_type {

       FileType::Directory => {
            "📁".to_string()
        }


        _ => {

            match file
                .name
                .split('.')
                .last()
                .unwrap_or("")
            {

                "png" | "jpg" | "jpeg" | "gif" => {
                    "🖼".to_string()
                }


                "mp3" | "wav" | "flac" => {
                    "🎵".to_string()
                }


                "mp4" | "mkv" | "avi" => {
                    "🎬".to_string()
                }


                "rs" | "cpp" | "py" | "js" | "ts" => {
                    "💻".to_string()
                }


                "txt" | "md" => {
                    "📄".to_string()
                }


                _ => {
                    "📄".to_string()
                }
            }
        }
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

        file_type: format!("{:?}", file.file_type),

        size: format_size(file.size),

        modified: format_modified(file.modified),

        icon: get_icon(file),
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
use crate::core::error::NexaroError;
use crate::models::file_info::FileInfo;
use crate::models::file_type::FileType;
use std::cmp::Ordering;
use std::fs;
use std::path::Path;
use crate::core::metadata::service::MetadataService;

pub fn read_directory(
    path: &Path,
) -> Result<Vec<FileInfo>, NexaroError> {
    let mut files = Vec::new();
    let metadata_service = MetadataService::new();
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;

        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };

        let file_type = if metadata.is_dir() {
            FileType::Directory
        } else {
            FileType::File
        };

               files.push(FileInfo {
            name: entry.file_name().to_string_lossy().to_string(),
            path: entry.path(),
            file_type,
           size: metadata_service.size(&entry.path()),
           modified: metadata_service.modified(&entry.path()),
           created: metadata_service.created(&entry.path()),
           extension: metadata_service.extension(&entry.path()),
           is_hidden: metadata_service.is_hidden(&entry.path()),
        });
    }

    files.sort_by(|a, b| {
        match (&a.file_type, &b.file_type) {
            (FileType::Directory, FileType::File) => Ordering::Less,
            (FileType::File, FileType::Directory) => Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    Ok(files)
}
  
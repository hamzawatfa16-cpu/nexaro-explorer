use std::path::Path;
use std::fs;
use std::time::SystemTime;


pub struct MetadataService;


impl MetadataService {

    pub fn new() -> Self {
        Self
    }


    pub fn extension(
        &self,
        path: &Path,
    ) -> Option<String> {

        path.extension()
            .map(|ext| {
                ext.to_string_lossy()
                    .to_string()
            })
    }


    pub fn is_hidden(
        &self,
        path: &Path,
    ) -> bool {

        path.file_name()
            .map(|name| {
                name.to_string_lossy()
                    .starts_with(".")
            })
            .unwrap_or(false)
    }


    pub fn size(
        &self,
        path: &Path,
    ) -> u64 {

        fs::metadata(path)
            .map(|meta| meta.len())
            .unwrap_or(0)
    }


    pub fn created(
        &self,
        path: &Path,
    ) -> Option<SystemTime> {

        fs::metadata(path)
            .ok()
            .and_then(|meta| meta.created().ok())
    }


    pub fn modified(
        &self,
        path: &Path,
    ) -> Option<SystemTime> {

        fs::metadata(path)
            .ok()
            .and_then(|meta| meta.modified().ok())
    }
}
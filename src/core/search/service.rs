use crate::models::file_info::FileInfo;

pub struct SearchService;

impl SearchService {
    pub fn new() -> Self {
        Self
    }

    pub fn filter(
        &self,
        files: &[FileInfo],
        query: &str,
    ) -> Vec<FileInfo> {
        let query = query.to_lowercase();

        files
            .iter()
            .filter(|file| {
                file.name
                    .to_lowercase()
                    .contains(&query)
            })
            .cloned()
            .collect()
    }
}
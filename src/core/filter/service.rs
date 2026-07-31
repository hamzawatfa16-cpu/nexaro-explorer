use crate::models::file_info::FileInfo;


pub struct FilterService;


impl FilterService {

    pub fn new() -> Self {
        Self
    }


    pub fn hide_hidden_files(
        &self,
        files: Vec<FileInfo>,
        show_hidden: bool,
    ) -> Vec<FileInfo> {

        if show_hidden {
            return files;
        }

        files
            .into_iter()
            .filter(|file| !file.is_hidden)
            .collect()
    }
}
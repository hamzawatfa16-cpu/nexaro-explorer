use crate::models::file_info::FileInfo;
use crate::models::file_type::FileType;

use std::path::PathBuf;


pub struct DriveService;


impl DriveService {


    pub fn list_drives() -> Vec<FileInfo> {

        let mut drives = Vec::new();


        for letter in 'A'..='Z' {

            let path = PathBuf::from(
                format!("{}:\\", letter)
            );


            if path.exists() {

                drives.push(FileInfo {

                    name: format!("{}:", letter),

                    path,

                    file_type: FileType::Directory,

                    size: 0,

                    modified: None,

                    created: None,

                    extension: None,

                    is_hidden: false,
                });
            }
        }


        drives
    }
}
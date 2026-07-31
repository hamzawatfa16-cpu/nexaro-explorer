use crate::models::file_info::FileInfo;

#[derive(Debug, Clone)]
pub enum SortMode {
    NameAscending,
    NameDescending,
    SizeAscending,
    SizeDescending,
    ModifiedNewest,
    ModifiedOldest,
}


pub struct SortService;


impl SortService {

    pub fn new() -> Self {
        Self
    }


    pub fn sort(
        &self,
        files: &mut Vec<FileInfo>,
        mode: SortMode,
    ) {

        match mode {

            SortMode::NameAscending => {
                files.sort_by(|a, b| {
                    a.name
                        .to_lowercase()
                        .cmp(&b.name.to_lowercase())
                });
            }


            SortMode::NameDescending => {
                files.sort_by(|a, b| {
                    b.name
                        .to_lowercase()
                        .cmp(&a.name.to_lowercase())
                });
            }


            SortMode::SizeAscending => {
                files.sort_by(|a, b| {
                    a.size.cmp(&b.size)
                });
            }


            SortMode::SizeDescending => {
                files.sort_by(|a, b| {
                    b.size.cmp(&a.size)
                });
            }


            SortMode::ModifiedNewest => {
                files.sort_by(|a, b| {
                    b.modified.cmp(&a.modified)
                });
            }


            SortMode::ModifiedOldest => {
                files.sort_by(|a, b| {
                    a.modified.cmp(&b.modified)
                });
            }
        }
    }
}
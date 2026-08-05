use slint::Image;

#[derive(Clone, Debug)]
pub struct FileItem {
    pub name: String,
    pub path: String,
    pub file_type: String,
    pub size: String,
    pub modified: String,
    pub icon: Image,
    pub is_directory: bool,
}
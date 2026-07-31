use crate::models::file_info::FileInfo;
use crate::models::location::ExplorerLocation;

use std::path::PathBuf;


pub struct ExplorerState {

    pub location: ExplorerLocation,

    pub files: Vec<FileInfo>,

    pub selected: Vec<PathBuf>,
}


impl ExplorerState {


    pub fn new(path: PathBuf) -> Self {

        Self {

            location: ExplorerLocation::Folder(path),

            files: Vec::new(),

            selected: Vec::new(),
        }
    }



    pub fn set_location(
        &mut self,
        location: ExplorerLocation,
    ) {

        self.location = location;
    }



    pub fn set_files(
        &mut self,
        files: Vec<FileInfo>,
    ) {

        self.files = files;
    }



    pub fn select(
        &mut self,
        path: PathBuf,
    ) {

        if !self.selected.contains(&path) {

            self.selected.push(path);
        }
    }



    pub fn clear_selection(
        &mut self,
    ) {

        self.selected.clear();
    }



    pub fn files(
        &self,
    ) -> &[FileInfo] {

        &self.files
    }

pub fn selected_files(
    &self,
) -> &[PathBuf] {

    &self.selected
}

}
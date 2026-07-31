use crate::core::error::NexaroError;
use crate::core::explorer::service::ExplorerService;

use std::path::PathBuf;


pub struct CommandService<'a> {
    explorer: &'a mut ExplorerService,
}


impl<'a> CommandService<'a> {

    pub fn new(
        explorer: &'a mut ExplorerService,
    ) -> Self {
        Self {
            explorer,
        }
    }


    pub fn copy_selected(
        &mut self,
    ) {
        let files = self
            .explorer
            .selected_files()
            .to_vec();

        self.explorer
            .copy_files(files);
    }


    pub fn cut_selected(
        &mut self,
    ) {
        let files = self
            .explorer
            .selected_files()
            .to_vec();

        self.explorer
            .cut_files(files);
    }


    pub fn paste(
        &mut self,
        destination: PathBuf,
    ) -> Result<(), NexaroError> {

        self.explorer
            .paste(destination)
    }


    pub fn delete_selected(
        &mut self,
    ) -> Result<(), NexaroError> {

        let files = self
            .explorer
            .selected_files()
            .to_vec();

        for file in files {
         self.explorer
    .delete(&file)?;
        }

        self.explorer
            .clear_selection();

        Ok(())
    }

pub fn rename(
    &mut self,
    old_path: PathBuf,
    new_path: PathBuf,
) -> Result<(), NexaroError> {

    self.explorer
        .rename(
            &old_path,
            &new_path,
        )
}


pub fn create_folder(
    &mut self,
    path: PathBuf,
) -> Result<(), NexaroError> {

    self.explorer
        .create_folder(&path)
}


pub fn refresh(
    &self,
) -> Result<Vec<crate::models::file_info::FileInfo>, NexaroError> {

    self.explorer
        .refresh()
}

}
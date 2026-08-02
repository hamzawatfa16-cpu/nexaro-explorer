use crate::app::state::explorer_state::ExplorerState;
use crate::core::commands::service::CommandService;
use crate::core::drives::service::DriveService;
use crate::core::error::NexaroError;
use crate::core::explorer::service::ExplorerService;
use crate::models::file_info::FileInfo;
use crate::models::location::ExplorerLocation;
use std::path::PathBuf;

pub struct ExplorerController {
    explorer: ExplorerService,
    state: ExplorerState,
}

impl ExplorerController {
    pub fn open_this_pc(&mut self) {
        let drives = DriveService::list_drives();

        self.state
            .set_location(crate::models::location::ExplorerLocation::ThisPc);

        self.state.set_files(drives);
    }

    pub fn new(path: PathBuf) -> Self {
        Self {
            explorer: ExplorerService::new(path.clone()),
            state: ExplorerState::new(path),
        }
    }

    pub fn select_file(&mut self, path: PathBuf) {
        self.explorer.select_file(path.clone());
        self.state.select(path);
    }

    pub fn select_all(&mut self) {
        self.clear_selection();

        let paths: Vec<PathBuf> = self.state.files().iter().map(|f| f.path.clone()).collect();

        for path in paths {
            self.explorer.select_file(path.clone());
            self.state.select(path);
        }
    }

    pub fn refresh(&mut self) -> Result<(), NexaroError> {
        let files = self.explorer.refresh()?;

        self.state.set_location(ExplorerLocation::Folder(
            self.explorer.current_path().to_path_buf(),
        ));

        self.state.set_files(files);

        Ok(())
    }

    pub fn search(&mut self, query: &str) -> Result<(), NexaroError> {
        let files = self.explorer.refresh()?;

        let filtered = self.explorer.search_files(&files, query);

        self.state.set_files(filtered);

        Ok(())
    }

    pub fn state(&self) -> &ExplorerState {
        &self.state
    }

    pub fn files(&self) -> &[FileInfo] {
        self.state.files()
    }

    pub fn selected_files(&self) -> &[PathBuf] {
        self.state.selected_files()
    }

    pub fn open_folder(&mut self, path: PathBuf) {
        self.explorer.open_folder(path);
    }

    pub fn back(&mut self) {
        self.explorer.go_back();
    }

    pub fn forward(&mut self) {
        self.explorer.go_forward();
    }

    pub fn up(&mut self) {
        if let ExplorerLocation::Folder(path) = &self.state.location {
            if let Some(parent) = path.parent() {
                self.explorer.open_folder(parent.to_path_buf());
            }
        }
    }

    pub fn copy_selected(&mut self) {
        let mut command = CommandService::new(&mut self.explorer);
        command.copy_selected();
    }

    pub fn cut_selected(&mut self) {
        let mut command = CommandService::new(&mut self.explorer);
        command.cut_selected();
    }

    pub fn paste(&mut self) -> Result<(), NexaroError> {
        if let ExplorerLocation::Folder(path) = &self.state.location {
            let mut command = CommandService::new(&mut self.explorer);
            command.paste(path.clone())
        } else {
            Ok(())
        }
    }

    pub fn clear_selection(&mut self) {
        self.explorer.clear_selection();
        self.state.clear_selection();
    }

    pub fn delete_selected(&mut self) -> Result<(), NexaroError> {
        let mut command = CommandService::new(&mut self.explorer);

        let result = command.delete_selected();

        if result.is_ok() {
            self.state.clear_selection();
        }

        result
    }

    pub fn create_folder(&mut self, path: PathBuf) -> Result<(), NexaroError> {
        let mut command = CommandService::new(&mut self.explorer);
        command.create_folder(path)
    }

    pub fn rename(&mut self, old_path: PathBuf, new_path: PathBuf) -> Result<(), NexaroError> {
        let mut command = CommandService::new(&mut self.explorer);
        command.rename(old_path, new_path)
    }
}

use crate::core::clipboard::service::{
    ClipboardAction,
    ClipboardService,
};
use crate::core::selection::service::SelectionService;
use crate::core::error::NexaroError;
use crate::core::explorer::navigation::Navigation;
use crate::core::filesystem::FileSystem;
use crate::core::search::service::SearchService;
use crate::core::watcher::service::WatcherService;
use crate::models::file_info::FileInfo;
use crate::core::settings::service::SettingsService;
use notify::RecommendedWatcher;
use crate::core::filter::service::FilterService;
use std::path::{Path, PathBuf};
use crate::core::sort::service::{
    SortMode,
    SortService,
};

pub struct ExplorerService {
    filesystem: FileSystem,
    navigation: Navigation,
    search: SearchService,
    clipboard: ClipboardService,
    watcher: WatcherService,
    sorter: SortService,
    settings: SettingsService,
     filter: FilterService,
     selection: SelectionService,
}


impl ExplorerService {

    pub fn new(path: PathBuf) -> Self {
    let mut settings = SettingsService::new();

    settings.load();

    Self {
        filesystem: FileSystem::new(),
        navigation: Navigation::new(path),
        search: SearchService::new(),
        clipboard: ClipboardService::new(),
        watcher: WatcherService::new(),
        sorter: SortService::new(),
        settings,
        filter: FilterService::new(),
         selection: SelectionService::new(),
    }
}


pub fn refresh(
    &self,
) -> Result<Vec<FileInfo>, NexaroError> {

    let mut files = self.list_current_directory()?;

    files = self.filter_files(files);

    self.sort_files(
        &mut files,
        SortMode::NameAscending,
    );

    Ok(files)
}


pub fn filter_files(
    &self,
    files: Vec<FileInfo>,
) -> Vec<FileInfo> {

    self.filter.hide_hidden_files(
        files,
        self.settings
            .get()
            .show_hidden_files,
    )
}

pub fn settings(&self) -> &SettingsService {
    &self.settings
}


pub fn save_settings(&self) {
    self.settings.save();
}


pub fn sort_files(
    &self,
    files: &mut Vec<FileInfo>,
    mode: SortMode,
) {
    self.sorter.sort(files, mode);
}

    pub fn paste(
        &mut self,
        destination: PathBuf,
    ) -> Result<(), NexaroError> {

        let files = self.clipboard.files().to_vec();

        match self.clipboard.action() {

            Some(ClipboardAction::Copy) => {

                for file in files {

                    let name = file
                        .file_name()
                        .unwrap();

                    let target = destination.join(name);

                    self.filesystem.copy(
                        &file,
                        &target,
                    )?;
                }
            }


            Some(ClipboardAction::Cut) => {

                for file in files {

                    let name = file
                        .file_name()
                        .unwrap();

                    let target = destination.join(name);

                    self.filesystem.move_item(
                        &file,
                        &target,
                    )?;
                }

                self.clipboard.clear();
            }


            None => {}
        }

        Ok(())
    }


    pub fn current_path(&self) -> &Path {
        self.navigation.current()
    }


    pub fn list_current_directory(
        &self,
    ) -> Result<Vec<FileInfo>, NexaroError> {

        self.filesystem
            .list_directory(self.navigation.current())
    }


    pub fn open_folder(
        &mut self,
        path: PathBuf,
    ) {
        self.navigation.go_to(path);
    }


    pub fn go_back(&mut self) {
        self.navigation.back();
    }


    pub fn go_forward(&mut self) {
        self.navigation.forward();
    }


    pub fn search_files(
        &self,
        files: &[FileInfo],
        query: &str,
    ) -> Vec<FileInfo> {

        self.search.filter(files, query)
    }


    pub fn copy_files(
        &mut self,
        files: Vec<PathBuf>,
    ) {
        self.clipboard
            .set(files, ClipboardAction::Copy);
    }


    pub fn cut_files(
        &mut self,
        files: Vec<PathBuf>,
    ) {
        self.clipboard
            .set(files, ClipboardAction::Cut);
    }


    pub fn watch_current_directory(
        &self,
    ) -> notify::Result<RecommendedWatcher> {

        self.watcher.watch(
            self.navigation.current(),
            |event| {
                println!("Explorer change: {:?}", event);
            },
        )
    }



    pub fn select_file(
    &mut self,
    path: PathBuf,
) {
    self.selection.select(path);
}


pub fn deselect_file(
    &mut self,
    path: &PathBuf,
) {
    self.selection.deselect(path);
}


pub fn clear_selection(
    &mut self,
) {
    self.selection.clear();
}


pub fn selected_files(
    &self,
) -> &[PathBuf] {
    self.selection.selected()
}


pub fn create_folder(
    &self,
    path: &Path,
) -> Result<(), NexaroError> {

    self.filesystem
        .create_folder(path)?;

    Ok(())
}


pub fn rename(
    &self,
    old_path: &Path,
    new_path: &Path,
) -> Result<(), NexaroError> {

    self.filesystem
        .rename(old_path, new_path)?;

    Ok(())
}


pub fn delete(
    &self,
    path: &Path,
) -> Result<(), NexaroError> {

    self.filesystem
        .delete(path)?;

    Ok(())
}

}
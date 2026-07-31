use crate::core::settings::storage::SettingsStorage;

use std::path::PathBuf;


#[derive(Debug, Clone)]
pub struct Settings {
    pub default_folder: PathBuf,
    pub show_hidden_files: bool,
}


pub struct SettingsService {
    settings: Settings,
}


impl SettingsService {

    pub fn new() -> Self {
        Self {
            settings: Settings {
                default_folder: std::env::current_dir()
                    .unwrap_or_else(|_| PathBuf::from(".")),
                show_hidden_files: false,
            },
        }
    }


    pub fn load(&mut self) {

        if let Ok(data) = SettingsStorage::load() {

            if data.contains("\"show_hidden_files\": true") {
                self.settings.show_hidden_files = true;
            }
        }
    }


    pub fn save(&self) {

        let _ = SettingsStorage::save(
            &self.settings
        );
    }


    pub fn get(&self) -> &Settings {
        &self.settings
    }


    pub fn set_default_folder(
        &mut self,
        path: PathBuf,
    ) {
        self.settings.default_folder = path;
    }


    pub fn set_show_hidden_files(
        &mut self,
        value: bool,
    ) {
        self.settings.show_hidden_files = value;
    }
}
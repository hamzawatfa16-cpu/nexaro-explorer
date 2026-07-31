use crate::core::settings::service::Settings;

use std::fs;
use std::io;
use std::path::PathBuf;


pub struct SettingsStorage;


impl SettingsStorage {

    pub fn path() -> PathBuf {
        let mut path = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."));

        path.push("nexaro_settings.json");

        path
    }


    pub fn save(
        settings: &Settings,
    ) -> io::Result<()> {

        let data = format!(
            "{{\n  \"show_hidden_files\": {}\n}}",
            settings.show_hidden_files
        );

        fs::write(
            Self::path(),
            data,
        )
    }


    pub fn load() -> io::Result<String> {

        fs::read_to_string(
            Self::path()
        )
    }
}
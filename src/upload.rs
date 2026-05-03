use crate::app::App;
use std::{fs, path};

impl App {
    pub fn does_token_exist(&self) -> bool {
        let data_folder = dirs::data_dir()
            .expect("failed to find data path...")
            .join("al-goma/");

        if data_folder.try_exists().is_err() {
            fs::create_dir_all(data_folder.clone()).expect("failed to make config dir...");
            return false;
        };

        if data_folder.join("t.bin").exists() {
            return true;
        };

        false
    }
}

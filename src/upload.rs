use crate::app::App;
use std::fs;

impl App {}

pub fn does_token_exist() -> Result<bool, &'static str> {
    let data_folder = dirs::data_dir()
        .expect("failed to find data path...")
        .join("al-goma/");

    match data_folder.try_exists() {
        Ok(true) => match data_folder.join("token.bin").try_exists() {
            Ok(true) => return Ok(true),
            Ok(false) => return Ok(false),
            Err(_) => return Err("token.bin is corrupt!"),
        },
        Ok(false) => fs::create_dir_all(data_folder).expect("failed to make config dir..."),
        Err(_) => return Err("failed to find data path..."),
    };

    Ok(false)
}

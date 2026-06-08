use crate::{
    app::{App, AppState},
    dishtabase::items::Database,
};

use std::fs;

impl App {
    pub fn start_editing_dish(&mut self) {
        if self.db.dishes.is_empty() {
            return;
        }
        self.state = AppState::EditingDish;
    }

    pub fn confirm_editing_dish(&mut self) {
        self.state = AppState::EditingIngredient;
        self.pending_dish = Some(self.db.dishes[self.db_cursor.cursor].to_owned());
    }

    pub fn delete_dish_option(&mut self) {
        if self.ays_cursor == 0 {
            self.delete_dish();
        } else {
            self.state = AppState::ViewingDatabase
        }
    }
}
pub fn load() -> Database {
    let config_folder = dirs::config_dir()
        .expect("failed to find config path...")
        .join("al-goma/");

    let contents = match fs::read_to_string(config_folder.join("dishes.toml")) {
        Ok(s) => s,
        Err(_) => return Database { dishes: vec![] },
    };
    toml::from_str(&contents).expect("dishes.toml is fucked!")
}

pub fn save(db: &Database) {
    let config_folder = dirs::config_dir()
        .expect("failed to find config path...")
        .join("al-goma/");

    let contents = toml::to_string(db).expect("failed to serialize...");

    fs::create_dir_all(config_folder.clone()).expect("failed to make dir: .config");
    fs::write(config_folder.join("dishes.toml"), contents).expect("failed to write file...")
}

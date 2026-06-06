use crate::items::{self, Category, Database, Dish};
use crate::locale::UiText;
use crate::ui::Cursor;
use crate::upload::UploadProgress;
use crate::{db, items::Ingredient};
use crate::{list, locale};
use crate::{ui, upload};
use arboard::Clipboard;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{char, fs};
use tokio::sync::mpsc;

#[derive(Debug, PartialEq)]
pub enum Space {
    MainLeft,
    MainRight,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum Language {
    Eng,
    Swe,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub language: Language,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AppState {
    Error,
    Normal,
    MovingFocus,

    EnteringDishName,
    EnteringIngredients,
    PickingCategory,

    ViewingDatabase,
    EditingDish,
    EditingIngredient,
    EditingAddIngredient,
    EditingDishName,
    AreYouSureDelDish,

    NewList,
    ReplaceList,
    ShowGeneratedList,
    AddToGeneratedList,
    ShowShoppingList,
    AddToShoppingList,
    PromptPrint,

    UploadMenu,
    UploadFirstLogin,
    UploadWaitingForLoginUrl,
    UploadShowLoginUrl,
    UploadEnterCode,
    UploadLogginginWait,
    Uploading,
    UploadDone,
}

#[derive(Debug)]
pub struct App {
    pub current_dish_list: Option<Vec<Dish>>,
    pub shopping_list: Vec<Ingredient>,

    pub text_options: (bool, bool),
    pub text: HashMap<UiText, &'static str>,

    pub cursor: usize,
    pub db_cursor: Cursor,
    pub edit_cursor: Cursor,
    pub ays_cursor: usize,
    pub picking_cursor: usize,

    pub moving_focus: bool,
    pub selected_space: Space,

    pub left_window_actions: Vec<&'static str>,
    pub db: Database,
    pub state: AppState,
    pub prev_state: Option<AppState>,

    pub input: String,
    pub inline_complete: Option<String>,
    pub inline_complete_whole_word: Option<String>,
    pub default_upload_name: bool,

    pub pending_dish: Option<Dish>,
    pub category_db: HashMap<String, Category>,
    pub normalized_category_db: HashMap<String, Category>,
    pub non_normalized_category_db: HashMap<String, Category>,

    pub err_msg: Option<String>,

    pub url_receiver: Option<mpsc::Receiver<String>>,
    pub code_sender: Option<mpsc::Sender<String>>,
    pub login_result_receiver: Option<mpsc::Receiver<Result<(), String>>>,
    pub progress_checker_receiver: Option<mpsc::Receiver<upload::UploadProgress>>,
    pub upload_result_receiver: Option<mpsc::Receiver<Result<(), String>>>,

    pub login_url: Option<String>,
    pub progress: upload::UploadProgress,
}

impl App {
    pub fn init() -> Self {
        let text = match load_settings().language {
            Language::Eng => locale::english(),
            Language::Swe => locale::swedish(),
        };

        let left_window_actions = vec![
            text[&UiText::NewList],
            text[&UiText::ViewEditList],
            text[&UiText::AddToDishtabase],
            text[&UiText::ViewEditDishtabase],
            text[&UiText::Upload],
        ];

        let ingredient_category_db = items::build_ingredient_database();

        let load_shopping_list = list::load_shopping_list_config();

        let load_current_dish_list: Option<Vec<Dish>> = if load_shopping_list.is_empty() {
            None
        } else {
            list::load()
        };

        App {
            current_dish_list: load_current_dish_list,
            shopping_list: load_shopping_list,
            text_options: (false, false),
            text,
            cursor: 0,

            db_cursor: Cursor {
                cursor: 0,
                scroll: 0,
                visable_lines: 0,
            },

            edit_cursor: Cursor {
                cursor: 0,
                scroll: 0,
                visable_lines: 0,
            },
            ays_cursor: 0,
            picking_cursor: 0,
            moving_focus: false,
            selected_space: Space::MainLeft,
            db: db::load(),

            category_db: ingredient_category_db.clone(),

            normalized_category_db: ingredient_category_db
                .clone()
                .into_iter()
                .map(|(k, v)| (k.replace(' ', ""), v))
                .collect(),

            non_normalized_category_db: ingredient_category_db,
            state: AppState::Normal,
            prev_state: None,
            input: String::new(),
            default_upload_name: true,
            inline_complete: None,
            inline_complete_whole_word: None,
            pending_dish: None,
            left_window_actions,
            err_msg: None,
            url_receiver: None,
            code_sender: None,
            login_result_receiver: None,
            progress_checker_receiver: None,
            upload_result_receiver: None,
            login_url: None,
            progress: UploadProgress {
                procent: 0.0,
                done: false,
            },
        }
    }

    pub fn char_input(&mut self, c: char) {
        match self.state {
            AppState::EnteringDishName
            | AppState::EditingDishName
            | AppState::NewList
            | AppState::UploadEnterCode => self.input.push(c),

            AppState::EnteringIngredients
            | AppState::EditingIngredient
            | AppState::EditingAddIngredient
            | AppState::AddToShoppingList => {
                self.input.push(c);
                self.update_inline_complete_ingredients()
            }
            AppState::UploadMenu => {
                self.clear_and_push_input_upload(c);
            }

            _ => {}
        }
    }

    fn clear_and_push_input_upload(&mut self, c: char) {
        if self.default_upload_name == true {
            self.default_upload_name = false;
            self.input.clear();
            self.input.push(c);
        } else {
            self.input.push(c);
        }
    }

    pub fn backspace(&mut self, ctrl: bool) {
        match self.state {
            AppState::EnteringDishName
            | AppState::EditingDishName
            | AppState::NewList
            | AppState::UploadEnterCode
            | AppState::UploadMenu => {
                if ctrl == false {
                    self.input.pop();
                } else {
                    self.backspace_to_delimiter_or_whitespace()
                }
            }

            AppState::EnteringIngredients
            | AppState::EditingIngredient
            | AppState::EditingAddIngredient
            | AppState::AddToShoppingList => {
                if ctrl == false {
                    self.input.pop();
                    self.update_inline_complete_ingredients();
                } else {
                    self.backspace_to_delimiter_or_whitespace();
                    self.update_inline_complete_ingredients()
                }
            }
            _ => {}
        }
    }

    fn backspace_to_delimiter_or_whitespace(&mut self) {
        if let Some(c) = self.input.chars().rev().next() {
            if c.is_whitespace() || c == '.' || c == '-' || c == '_' {
                self.input.pop();
            } else {
                match self
                    .input
                    .rfind(|c: char| c.is_whitespace() || c == '.' || c == '-' || c == '_')
                {
                    Some(i) => self.input.truncate(i + 1),
                    None => self.input.clear(),
                }
            }
        }
    }

    fn update_inline_complete_ingredients(&mut self) {
        self.inline_complete = None;
        self.inline_complete_whole_word = None;
        for key in self.non_normalized_category_db.keys() {
            if key.starts_with(&self.input.to_lowercase()) {
                self.inline_complete_whole_word = Some(key.clone());
                let inline: &str = &key[self.input.len()..];
                self.inline_complete = Some(inline.to_string());
                break;
            }
        }
        if self.input.is_empty() {
            self.inline_complete = None;
            self.inline_complete_whole_word = None
        }
    }

    fn tab_complete(&mut self) {
        if let Some(inline_complete) = self.inline_complete_whole_word.clone() {
            self.input = inline_complete;
        }
        self.inline_complete = None;
        self.inline_complete_whole_word = None
    }

    pub fn handle_enter(&mut self) {
        if self.moving_focus {
            self.prev_state = Some(self.state);
            self.state = AppState::MovingFocus;
            self.db_cursor.cursor = 0;
            self.db_cursor.scroll = 0;
        }

        match self.state {
            // Main menu
            AppState::Normal | AppState::MovingFocus => {
                self.input.clear();
                if self.selected_space == Space::MainLeft {
                    match self.cursor {
                        0 => self.open_new_list(),
                        1 => self.open_view_edit_list(),
                        2 => self.open_add_dish_to_dishtabase(),
                        3 => self.open_view_edit_dishtabase(),
                        4 => self.open_upload(),
                        _ => {}
                    }
                    self.moving_focus = false;
                }
            }

            // List
            AppState::NewList => self.start_new_list(),
            AppState::ReplaceList => self.replace_old_list(),
            AppState::AddToGeneratedList => self.manual_add_dish_to_shoppinglist(),
            AppState::ShowGeneratedList => self.show_and_save_generated_list(),
            AppState::AddToShoppingList => self.add_to_shopping_list(),
            AppState::PromptPrint => self.print_promt_confirm_options(),
            AppState::EnteringDishName => self.confirm_dish_name(),
            AppState::EnteringIngredients => self.confirm_ingredient(),

            // Dishtabase
            AppState::ViewingDatabase => self.start_editing_dish(),
            AppState::EditingDish => self.confirm_editing_dish(),
            AppState::EditingIngredient => self.edit_ingredient(),
            AppState::EditingDishName => self.edit_dish_name(),
            AppState::AreYouSureDelDish => self.delete_dish_option(),
            AppState::EditingAddIngredient => self.edit_add_ingredient(),
            AppState::PickingCategory => self.confim_category(),

            // Upload
            AppState::UploadFirstLogin => self.upload_first_login(),
            AppState::UploadShowLoginUrl => self.state = AppState::UploadEnterCode,
            AppState::UploadEnterCode => self.send_code(),
            AppState::UploadMenu => self.init_upload_list(),

            _ => {}
        }
    }

    pub fn handle_esc(&mut self) {
        match self.state {
            AppState::EditingDish | AppState::AreYouSureDelDish => {
                self.db.dishes[self.db_cursor.cursor]
                    .ingredients
                    .sort_by_key(|c| c.category);
                self.state = AppState::ViewingDatabase;
                self.edit_cursor.cursor = 0;
                self.ays_cursor = 0;
                self.edit_cursor.scroll = 0;
            }
            AppState::EditingIngredient | AppState::EditingDishName => {
                self.state = AppState::EditingDish;
                self.pending_dish = None;
                self.input.clear();
            }
            AppState::PickingCategory => {
                if let Some(prev_state) = self.prev_state {
                    self.state = prev_state;
                    self.prev_state = None
                } else {
                    self.state = AppState::Normal
                }
            }
            AppState::AddToShoppingList => {
                self.state = AppState::ShowShoppingList;
                self.input.clear();
            }
            AppState::AddToGeneratedList => {
                self.state = self.prev_state.unwrap();
                self.prev_state = None;
                self.db_cursor.cursor = 0;
            }
            _ => {
                self.state = AppState::Normal;
                self.selected_space = Space::MainLeft;

                self.pending_dish = None;

                self.input.clear();
                self.cursor = 0;
                self.db_cursor.cursor = 0;
                self.edit_cursor.cursor = 0;
            }
        }
    }

    pub fn handle_delete(&mut self) {
        match self.state {
            AppState::EnteringIngredients | AppState::EditingDish | AppState::ShowShoppingList => {
                self.delete_ingredient()
            }
            AppState::ViewingDatabase if !self.db.dishes.is_empty() => {
                self.state = AppState::AreYouSureDelDish;
            }
            AppState::ShowGeneratedList => self.generate_new_dish(),
            _ => {}
        }
    }

    pub fn handle_tab(&mut self) {
        match self.state {
            AppState::EnteringDishName
            | AppState::EnteringIngredients
            | AppState::EditingIngredient
            | AppState::EditingDishName
            | AppState::EditingAddIngredient
            | AppState::NewList
            | AppState::AddToShoppingList
            | AppState::UploadEnterCode => self.tab_complete(),
            _ => {}
        }
    }

    pub fn push_dish_to_db(&mut self) {
        if self.cursor == 4 {
            return;
        }

        if self.pending_dish.is_some() {
            let dish = self.pending_dish.as_mut();
            let d = dish.unwrap();
            d.ingredients.sort_by_key(|c| c.category);
        }

        if let Some(dish) = self.pending_dish.take() {
            self.db.dishes.push(dish);
            db::save(&self.db);
        }

        self.state = AppState::EnteringDishName;
        self.input.clear();
    }

    fn manual_add_dish_to_shoppinglist(&mut self) {
        if let Some(dish_list) = self.current_dish_list.as_mut() {
            dish_list.push(self.db.dishes[self.db_cursor.cursor].clone());
        }
        self.state = self.prev_state.unwrap();
        self.prev_state = None;
    }

    fn add_to_shopping_list(&mut self) {
        let found_category = self.find_category(self.input.clone());
        let input = uppercase_words(&self.input.clone());
        let seach_input = input.clone();

        self.shopping_list.push(Ingredient {
            name: input,
            category: found_category,
            frozen: false,
        });

        if found_category == Category::Misc {
            self.prev_state = Some(self.state);
            self.state = AppState::PickingCategory
        }

        self.input.clear();
        self.shopping_list.sort_by_key(|i| i.category);

        for (i, ing) in self.shopping_list.iter().enumerate() {
            if ing.name == seach_input {
                self.db_cursor.cursor = i;
                ui::update_scroll(&mut self.db_cursor)
            }
        }

        if self.current_dish_list.is_none() {
            self.current_dish_list = Some(vec![Dish {
                name: "n/a".to_string(),
                ingredients: vec![],
            }]);
        }

        list::save_shopping_list_config(self.shopping_list.clone());
    }

    fn edit_ingredient(&mut self) {
        let found_category = self.find_category(self.input.clone());
        let input = uppercase_words(&self.input.clone());

        if let Some(pending_dish) = self.pending_dish.as_mut() {
            pending_dish.ingredients[self.edit_cursor.cursor]
                .name
                .clear();
            pending_dish.ingredients[self.edit_cursor.cursor].name = input;
            pending_dish.ingredients[self.edit_cursor.cursor].category = found_category;

            self.db.dishes[self.db_cursor.cursor] = pending_dish.clone();
            db::save(&self.db);

            self.pending_dish = None;
            self.input.clear();
        }

        self.state = AppState::EditingDish
    }

    pub fn edit_add_ingredient(&mut self) {
        self.pending_dish = Some(self.db.dishes[self.db_cursor.cursor].clone());

        let input = uppercase_words(&self.input.clone());
        let found_category = self.find_category(input.clone());

        if let Some(pending_dish) = self.pending_dish.as_mut() {
            pending_dish.ingredients.push(Ingredient {
                name: input,
                category: found_category,
                frozen: false,
            });

            self.db.dishes[self.db_cursor.cursor] = pending_dish.clone();
            self.edit_cursor.cursor = self.db.dishes[self.db_cursor.cursor].ingredients.len() - 1;
            ui::update_scroll(&mut self.edit_cursor);
            self.state = AppState::EditingDish
        }
        self.input.clear();
        db::save(&self.db);
        self.pending_dish = None;
    }

    pub fn edit_dish_name(&mut self) {
        if self.state != AppState::EditingDishName {
            return;
        }

        if let Some(pending_dish) = self.pending_dish.as_mut() {
            let input = uppercase_words(&self.input.clone());

            pending_dish.name.clear();
            pending_dish.name = input;

            self.db.dishes[self.db_cursor.cursor] = pending_dish.clone();

            self.input.clear();
            db::save(&self.db);
            self.pending_dish = None;
            self.state = AppState::EditingDish
        }
    }

    pub fn delete_dish(&mut self) {
        if self.db.dishes.is_empty() {
            return;
        }
        self.db.dishes.remove(self.db_cursor.cursor);

        self.ays_cursor = 0;

        if self.db_cursor.cursor == self.db.dishes.len() && !self.db.dishes.is_empty() {
            self.db_cursor.cursor -= 1;
        }

        db::save(&self.db);
        self.state = AppState::ViewingDatabase;
    }

    fn delete_ingredient(&mut self) {
        match self.state {
            AppState::EnteringIngredients => {
                if let Some(pending_dish) = self.pending_dish.as_mut() {
                    pending_dish.ingredients.pop();
                }
            }
            AppState::EditingDish => {
                self.pending_dish = Some(self.db.dishes[self.db_cursor.cursor].to_owned());

                if let Some(pending_dish) = self.pending_dish.as_mut() {
                    if !pending_dish.ingredients.is_empty() {
                        pending_dish.ingredients.remove(self.edit_cursor.cursor);
                    }

                    if self.edit_cursor.cursor == pending_dish.ingredients.len()
                        && !pending_dish.ingredients.is_empty()
                    {
                        self.edit_cursor.cursor -= 1;
                    }

                    self.db.dishes[self.db_cursor.cursor] = pending_dish.clone();
                    db::save(&self.db);
                    self.pending_dish = None;
                }
            }
            AppState::ShowShoppingList => {
                if !self.shopping_list.is_empty() {
                    self.shopping_list.remove(self.db_cursor.cursor);
                    if (self.db_cursor.cursor == self.shopping_list.len()
                        || self.db_cursor.cursor > self.shopping_list.len())
                        && !self.shopping_list.is_empty()
                    {
                        self.db_cursor.cursor = self.shopping_list.len() - 1;
                    }
                    if self.shopping_list.is_empty() {
                        self.current_dish_list = None
                    }
                }
                list::save_shopping_list_config(self.shopping_list.clone());
            }

            _ => {}
        }
    }

    fn confim_category(&mut self) {
        let chosen_category: Category = match self.picking_cursor {
            0 => Category::Misc,
            1 => Category::Vegetables,
            2 => Category::Fruit,
            3 => Category::Dairy,
            4 => Category::Protein,
            5 => Category::Pantry,
            6 => Category::Spices,
            _ => Category::Misc,
        };

        if let Some(pending_dish) = self.pending_dish.as_mut() {
            let mut i = pending_dish.ingredients.pop().unwrap();
            i.category = chosen_category;
            self.pending_dish
                .as_mut()
                .unwrap()
                .ingredients
                .push(i.clone());

            items::save_learned_categories(i);
            self.category_db = items::build_ingredient_database();
        }

        if let Some(prev_state) = self.prev_state {
            if prev_state == AppState::EditingDish {
                self.db.dishes[self.db_cursor.cursor].ingredients[self.edit_cursor.cursor]
                    .category = chosen_category;
            } else if prev_state == AppState::AddToShoppingList
                && let Some(mut i) = self.shopping_list.pop()
            {
                let search_input = i.name.clone();
                i.category = chosen_category;
                self.shopping_list.push(i);
                self.shopping_list.sort_by_key(|c| c.category);

                for (i, ing) in self.shopping_list.iter().enumerate() {
                    if ing.name == search_input {
                        self.db_cursor.cursor = i;
                        ui::update_scroll(&mut self.db_cursor)
                    }
                }

                list::save_shopping_list_config(self.shopping_list.clone());
            }

            self.state = prev_state;
        }
        self.prev_state = None;
        self.picking_cursor = 0
    }

    fn confirm_ingredient(&mut self) {
        let input = uppercase_words(&self.input.clone());
        let found_category = self.find_category(input.clone());

        if input.is_empty() {
            return;
        }

        if let Some(dish) = self.pending_dish.as_mut() {
            dish.ingredients.push(Ingredient {
                name: input,
                frozen: false,
                category: found_category,
            });
        }

        if found_category == Category::Misc {
            self.prev_state = Some(self.state);
            self.state = AppState::PickingCategory
        }

        self.inline_complete = None;
        self.inline_complete_whole_word = None;
        self.input.clear();
    }

    fn confirm_dish_name(&mut self) {
        let input = uppercase_words(&self.input.clone());

        if input.is_empty() {
            return;
        }

        self.pending_dish = Some(Dish {
            name: input,
            ingredients: vec![],
        });

        self.input.clear();

        self.state = AppState::EnteringIngredients
    }

    fn find_category(&mut self, look_up: String) -> Category {
        if let Some(c) = self.normalized_category_db.get(&look_up.to_lowercase()) {
            c.to_owned()
        } else if let Some(c) = self.category_db.get(&look_up.to_lowercase()) {
            c.to_owned()
        } else {
            Category::Misc
        }
    }

    pub fn text_get(&self, input: UiText) -> String {
        self.text.get(&input).expect("No Translation!").to_string()
    }

    pub fn print_shopping_list_txt_file(
        &self,
        shopping_list: Vec<Ingredient>,
        wants_index: bool,
        wants_categories: bool,
    ) -> std::io::Result<()> {
        let s = self.make_txt_string(shopping_list, wants_categories, wants_index);
        let date = Utc::now().date_naive();

        let shopping_list_folder: PathBuf = if dirs::document_dir().is_some() {
            dirs::document_dir()
                .expect("failed to find document path...")
                .join("shopping-lists/")
        } else {
            dirs::home_dir()
                .expect("failed to find document path...")
                .join("shopping-lists/")
        };

        let mut file_name = format!("shopping_list-{}.txt", date);

        fs::create_dir_all(shopping_list_folder.clone())?;

        if fs::exists(shopping_list_folder.join(file_name.clone())).unwrap() {
            let mut i = 2;
            file_name = format!("shopping_list-{}:{}.txt", date, i);
            while fs::exists(shopping_list_folder.join(file_name.clone())).unwrap() {
                i += 1;
                file_name = format!("shopping_list-{}:{}.txt", date, i);
            }
        }

        fs::write(shopping_list_folder.join(file_name), s)?;

        Ok(())
    }

    fn make_txt_string(
        &self,
        shopping_list: Vec<Ingredient>,
        cat_option: bool,
        i_option: bool,
    ) -> String {
        let mut output = String::new();
        let mut txt = String::new();
        let mut prev_category = Category::Vegetables;
        let mut veg_been_done = false;
        let mut cs = 0;
        let mut selected_cat: Category;

        for (i, ing) in shopping_list.iter().enumerate() {
            let mut space = "                         ".to_string();
            let i = i + 1;
            selected_cat = ing.category;

            if i_option {
                output.push_str(&i.to_string());
                output.push_str(". ");
            }

            output.push_str(&ing.name.to_string());

            for _ in ing.name.chars() {
                space.pop();
            }

            if i_option && i > 9 {
                space.pop();
            }

            if cat_option {
                if ing.category != prev_category
                    || (prev_category == Category::Vegetables && !veg_been_done)
                {
                    output.push_str(&space);
                    output.push_str(&self.get_category_name(ing.category));
                } else if ing.category == prev_category {
                    let cat_count = self.get_category_name(prev_category).len();

                    while cs < cat_count {
                        space.push(' ');
                        cs += 1
                    }

                    if selected_cat == ing.category {
                        space.pop();
                    }

                    let mut margin = "│";

                    if i < shopping_list.len() {
                        let next_cat = shopping_list[i].category;
                        if next_cat != ing.category {
                            space.drain(..2);
                            margin = "──┘";
                        }
                    } else if i == shopping_list.len() {
                        space.drain(..2);
                        margin = "──┘";
                    }

                    output.push_str(&space);
                    output.push_str(margin);
                }
            }
            cs = 0;
            prev_category = ing.category;
            veg_been_done = true;
            output.push('\n');
            txt.push_str(&output.clone());
            output.clear();
        }
        txt
    }
}

pub fn uppercase_words(data: &str) -> String {
    // Uppercase first letter in string, and letters after spaces.
    let mut result = String::new();
    let mut first = true;
    for value in data.chars() {
        if first {
            result.push(value.to_uppercase().next().unwrap());
            first = false;
        } else {
            result.push(value);
            if value == ' ' {
                first = true;
            }
        }
    }
    result
}

fn load_settings() -> Settings {
    let config_folder = dirs::config_dir()
        .expect("failed to find config path...")
        .join("al-goma/");

    let settings = match fs::read_to_string(config_folder.join("settings.toml")) {
        Ok(s) => s,
        Err(_) => {
            let default_settings = Settings {
                language: Language::Eng,
            };

            let default = toml::to_string(&default_settings).expect("failed to serialize...");

            fs::create_dir_all(config_folder.clone()).expect("failed to make dir config dir...");
            fs::write(config_folder.join("settings.toml"), default)
                .expect("failed to write file...");

            return default_settings;
        }
    };

    let settings_load: Settings = toml::from_str(&settings).expect("settings.toml is fucked!");

    settings_load
}

pub fn copy_to_clipboard(input: String) -> Result<(), String> {
    match Clipboard::new().unwrap().set_text(input) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn paste_from_clipboard() -> Result<String, String> {
    match Clipboard::new().unwrap().get_text() {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}

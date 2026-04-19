use crate::items::{self, Category, Database, Dish};
use crate::locale::UiText;
use crate::ui;
use crate::ui::Cursor;
use crate::{db, items::Ingredient};
use crate::{list, locale};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

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
    ShowShoppingList,
    AddToShoppingList,
    PromptPrint,
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
    pub pending_dish: Option<Dish>,
    pub category_db: HashMap<String, Category>,
    pub normalized_category_db: HashMap<String, Category>,
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
        ];

        let ingredient_category_db = match load_settings().language {
            Language::Eng => items::ingredient_category_db_eng(),
            Language::Swe => items::ingredient_category_db_swe(),
        };

        let load_shopping_list = list::load_shopping_list_config();

        let load_current_dish_list: Option<Vec<Dish>>;
        
        if load_shopping_list.is_empty() {
            load_current_dish_list = None;
        } else {
            load_current_dish_list = list::load()
        }

        App {
            current_dish_list: load_current_dish_list,
            shopping_list: load_shopping_list,
            text_options: (false, false),
            text: text,
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
                .into_iter()
                .map(|(k, v)| (k.replace(' ', ""), v))
                .collect(),

            state: AppState::Normal,
            prev_state: None,
            input: String::new(),
            pending_dish: None,
            left_window_actions: left_window_actions,
        }
    }

    pub fn keyboard_input(&mut self, c: char) {
        match self.state {
            AppState::EnteringDishName
            | AppState::EnteringIngredients
            | AppState::EditingIngredient
            | AppState::EditingDishName
            | AppState::EditingAddIngredient
            | AppState::NewList
            | AppState::AddToShoppingList => {
                self.input.push(c);
            }
            _ => {}
        }
    }

    pub fn backspace(&mut self) {
        match self.state {
            AppState::EnteringDishName
            | AppState::EnteringIngredients
            | AppState::EditingIngredient
            | AppState::EditingAddIngredient
            | AppState::EditingDishName
            | AppState::NewList
            | AppState::AddToShoppingList => {
                self.input.pop();
            }
            _ => {}
        }
    }

    pub fn handle_enter(&mut self) {
        if self.moving_focus {
            self.prev_state = Some(self.state);
            self.state = AppState::MovingFocus;
            self.db_cursor.cursor = 0;
            self.db_cursor.scroll = 0;
        }

        match self.state {
            AppState::Normal | AppState::MovingFocus => {
                if self.selected_space == Space::MainLeft && self.cursor == 0 {
                    if let Some(list) = self.current_dish_list.as_ref() {
                        if list.is_empty() {
                            self.state = AppState::NewList;
                            return;
                        } else {
                            self.state = AppState::ReplaceList;
                            self.selected_space = Space::MainRight;
                            self.moving_focus = false;

                            return;
                        }
                    }

                    self.state = AppState::NewList;
                    self.selected_space = Space::MainRight
                } else if self.selected_space == Space::MainLeft && self.cursor == 1 {
                    self.state = AppState::ShowShoppingList;
                    self.selected_space = Space::MainRight;
                } else if self.selected_space == Space::MainLeft && self.cursor == 2 {
                    self.state = AppState::EnteringDishName;
                    self.selected_space = Space::MainRight
                } else if self.selected_space == Space::MainLeft && self.cursor == 3 {
                    db::load();
                    self.state = AppState::ViewingDatabase;
                    self.selected_space = Space::MainRight;
                }
                self.moving_focus = false
            }
            AppState::NewList => {
                self.generate_list();
            }
            AppState::ReplaceList => {
                if self.ays_cursor == 0 {
                    self.current_dish_list = None;
                    self.state = AppState::NewList
                } else {
                    self.state = AppState::Normal;
                    self.selected_space = Space::MainLeft
                }
            }
            AppState::ShowGeneratedList => {
                self.state = AppState::ShowShoppingList;
                self.cursor = 1;
            }
            AppState::AddToShoppingList => {
                self.add_to_shopping_list();
            }
            AppState::PromptPrint => {
                if self.ays_cursor == 0 {
                    self.text_options.0 = !self.text_options.0;
                }
                if self.ays_cursor == 1 {
                    self.text_options.1 = !self.text_options.1;
                }
            }
            AppState::EnteringDishName => {
                self.confirm_dish_name();
            }
            AppState::EnteringIngredients => {
                self.confirm_ingredient();
            }
            AppState::ViewingDatabase => {
                if self.db.dishes.is_empty() {
                    return;
                }
                self.state = AppState::EditingDish;
            }
            AppState::EditingDish => {
                self.state = AppState::EditingIngredient;
                self.pending_dish = Some(self.db.dishes[self.db_cursor.cursor].to_owned());
            }
            AppState::EditingIngredient => self.edit_ingredient(),
            AppState::EditingDishName => self.edit_dish_name(),
            AppState::AreYouSureDelDish => {
                if self.ays_cursor == 0 {
                    self.delete_dish();
                } else {
                    self.state = AppState::ViewingDatabase
                }
            }
            AppState::EditingAddIngredient => self.edit_add_ingredient(),
            AppState::PickingCategory => self.confim_category(),
            _ => {}
        }
    }

    pub fn handle_esc(&mut self) {
        if matches!(self.state, AppState::EditingDish)
            || matches!(self.state, AppState::AreYouSureDelDish)
        {
            self.db.dishes[self.db_cursor.cursor]
                .ingredients
                .sort_by_key(|c| c.category);
            self.state = AppState::ViewingDatabase;
            self.edit_cursor.cursor = 0;
            self.ays_cursor = 0;
            self.edit_cursor.scroll = 0;
        } else if matches!(self.state, AppState::EditingIngredient)
            || matches!(self.state, AppState::EditingDishName)
        {
            self.state = AppState::EditingDish;
            self.pending_dish = None;
            self.input.clear();
        } else if matches!(self.state, AppState::PickingCategory) {
            if let Some(prev_state) = self.prev_state {
                self.state = prev_state;
                self.prev_state = None
            } else {
                self.state = AppState::Normal;
            }
        } else if matches!(self.state, AppState::AddToShoppingList) {
            self.state = AppState::ShowShoppingList;
            self.input.clear();
        } else {
            self.state = AppState::Normal;
            self.selected_space = Space::MainLeft;

            self.pending_dish = None;

            self.input.clear();
            self.cursor = 0;
            self.db_cursor.cursor = 0;
            self.edit_cursor.cursor = 0;
        }
    }

    pub fn handle_delete(&mut self) {
        match self.state {
            AppState::EnteringIngredients | AppState::EditingDish | AppState::ShowShoppingList => {
                self.delete_ingredient()
            }
            AppState::ViewingDatabase => {
                if !self.db.dishes.is_empty() {
                    self.state = AppState::AreYouSureDelDish;
                }
            }
            AppState::ShowGeneratedList => self.generate_new_dish(),
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

    fn delete_dish(&mut self) {
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
        let output = self.text.get(&input).expect("No Translation!").to_string();
        output
    }

    pub fn print_shopping_list_txt_file(
        &self,
        shopping_list: Vec<Ingredient>,
        wants_index: bool,
        wants_categories: bool,
    ) -> std::io::Result<()> {
        let s = self.make_txt_string(shopping_list, wants_categories, wants_index);
        let date = Utc::now().date_naive();
        let mut file_name = format!("Shopping-Lists/Shopping_List-{}.txt", date);

        fs::create_dir_all("Shopping-Lists/")?;

        if fs::exists(file_name.clone()).unwrap() {
            let mut i = 2;
            file_name = format!("Shopping-Lists/Shopping_List-{}({}).txt", date, i);
            while fs::exists(file_name.clone()).unwrap() {
                i += 1;
                file_name = format!("Shopping-Lists/Shopping_List-{}({}).txt", date, i);
            }
        }

        fs::write(file_name, s)?;

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
    let settings = match fs::read_to_string(".config/settings.toml") {
        Ok(s) => s,
        Err(_) => {
            let default_settings = Settings {
                language: Language::Eng,
            };

            let default = toml::to_string(&default_settings).expect("failed to serialize...");

            fs::create_dir_all(".config/").expect("failed to make dir: .config");
            fs::write(".config/settings.toml", default).expect("failed to write file...");

            return default_settings;
        }
    };

    let settings_load: Settings = toml::from_str(&settings).expect("settings.toml is fucked!");

    settings_load
}

use crate::items::{self, Category, Database, Dish};
use crate::{db, items::Ingredient};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Space {
    MainLeft,
    MainRight,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AppState {
    Normal,
    EnteringDishName,
    EnteringIngredients,
    PickingCategory,
    ViewingDatabase,
    EditingDish,
    EditingIngredient,
    EditingAddIngredient,
    EditingDishName,
    AreYouSureDelDish,
}

#[derive(Debug)]
pub struct App {
    pub list: Option<Vec<String>>,
    pub cursor: usize,
    pub db_cursor: usize,
    pub edit_cursor: usize,
    pub del_cursor: usize,
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
}

impl App {
    pub fn init() -> Self {
        App {
            list: None,
            cursor: 0,
            db_cursor: 0,
            edit_cursor: 0,
            del_cursor: 0,
            picking_cursor: 0,
            moving_focus: false,
            selected_space: Space::MainLeft,
            db: db::load(),
            category_db: items::ingredient_category_db(),
            state: AppState::Normal,
            prev_state: None,
            input: String::new(),
            pending_dish: None,
            left_window_actions: vec![
                "New List",
                "View/Edit List",
                "Add Dish",
                "Add Dish to Dishtabase",
                "View/Edit Dishtabase",
                "Upload",
            ],
        }
    }
    pub fn keyboard_input(&mut self, c: char) {
        match self.state {
            AppState::EnteringDishName
            | AppState::EnteringIngredients
            | AppState::EditingIngredient
            | AppState::EditingDishName
            | AppState::EditingAddIngredient => {
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
            | AppState::EditingAddIngredient => {
                self.input.pop();
            }
            _ => {}
        }
    }

    pub fn handle_enter(&mut self) {
        if self.moving_focus {
            if (self.selected_space == Space::MainLeft) && self.cursor == 3 {
                self.state = AppState::EnteringDishName;
                self.selected_space = Space::MainRight
            } else if self.selected_space == Space::MainLeft && self.cursor == 4 {
                db::load();

                self.state = AppState::ViewingDatabase;
                self.selected_space = Space::MainRight;
            }
            self.moving_focus = false
        } else {
            match self.state {
                AppState::Normal => {
                    if (self.selected_space == Space::MainLeft) && self.cursor == 3 {
                        self.state = AppState::EnteringDishName;
                        self.selected_space = Space::MainRight
                    } else if self.selected_space == Space::MainLeft && self.cursor == 4 {
                        db::load();
                        self.state = AppState::ViewingDatabase;
                        self.selected_space = Space::MainRight;
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
                    self.pending_dish = Some(self.db.dishes[self.db_cursor].to_owned());
                }
                AppState::EditingIngredient => self.edit_ingredient(),
                AppState::EditingDishName => self.edit_dish_name(),
                AppState::AreYouSureDelDish => {
                    if self.del_cursor == 0 {
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
    }

    pub fn handle_esc(&mut self) {
        if matches!(self.state, AppState::EditingDish)
            || matches!(self.state, AppState::AreYouSureDelDish)
        {
            self.db.dishes[self.db_cursor]
                .ingredients
                .sort_by_key(|c| c.category);
            self.state = AppState::ViewingDatabase;
            self.edit_cursor = 0;
            self.del_cursor = 0;
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
        } else {
            self.state = AppState::Normal;
            self.selected_space = Space::MainLeft;

            self.pending_dish = None;

            self.input.clear();
            self.cursor = 0;
        }
    }

    pub fn handle_delete(&mut self) {
        match self.state {
            AppState::EnteringIngredients | AppState::EditingDish => self.delete_ingredient(),
            AppState::ViewingDatabase => self.state = AppState::AreYouSureDelDish,
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

    fn edit_ingredient(&mut self) {
        let found_category = self.find_category(self.input.clone());
        let input = uppercase_words(&self.input.clone());

        if let Some(pending_dish) = self.pending_dish.as_mut() {
            pending_dish.ingredients[self.edit_cursor].name.clear();
            pending_dish.ingredients[self.edit_cursor].name = input;
            pending_dish.ingredients[self.edit_cursor].category = found_category;

            self.db.dishes[self.db_cursor] = pending_dish.clone();
            db::save(&self.db);

            self.pending_dish = None;
            self.input.clear();
        }

        self.state = AppState::EditingDish
    }

    pub fn edit_add_ingredient(&mut self) {
        self.pending_dish = Some(self.db.dishes[self.db_cursor].clone());
        let input = uppercase_words(&self.input.clone());
        let found_category = self.find_category(input.clone());
        if let Some(pending_dish) = self.pending_dish.as_mut() {
            pending_dish.ingredients.push(Ingredient {
                name: input,
                category: found_category,
                frozen: false,
            });

            self.db.dishes[self.db_cursor] = pending_dish.clone();
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

            self.db.dishes[self.db_cursor] = pending_dish.clone();

            self.input.clear();
            db::save(&self.db);
            self.pending_dish = None;
            self.state = AppState::EditingDish
        }
    }

    fn delete_dish(&mut self) {
        if self.db.dishes.len() == 0 {
            return;
        }
        self.db.dishes.remove(self.db_cursor);

        self.del_cursor = 0;

        if self.db_cursor == self.db.dishes.len() && self.db.dishes.len() > 0 {
            self.db_cursor -= 1;
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
                self.pending_dish = Some(self.db.dishes[self.db_cursor].to_owned());

                if let Some(pending_dish) = self.pending_dish.as_mut() {
                    if !pending_dish.ingredients.is_empty() {
                        pending_dish.ingredients.remove(self.edit_cursor);
                    }

                    if self.edit_cursor == pending_dish.ingredients.len()
                        && pending_dish.ingredients.len() > 0
                    {
                        self.edit_cursor -= 1;
                    }

                    self.db.dishes[self.db_cursor] = pending_dish.clone();
                    db::save(&self.db);
                    self.pending_dish = None;
                }
            }

            _ => {}
        }
    }

    fn confim_category(&mut self) {
        let chosen_category: Category;

        match self.picking_cursor {
            0 => chosen_category = Category::Misc,
            1 => chosen_category = Category::Vegtables,
            2 => chosen_category = Category::Fruit,
            3 => chosen_category = Category::Dairy,
            4 => chosen_category = Category::Protein,
            5 => chosen_category = Category::DryGoods,
            6 => chosen_category = Category::Spices,
            _ => chosen_category = Category::Misc,
        }
        if let Some(pending_dish) = self.pending_dish.as_mut() {
            let mut i = pending_dish.ingredients.pop().unwrap();
            i.category = chosen_category;
            self.pending_dish
                .as_mut()
                .unwrap()
                .ingredients
                .push(i.clone());
        }

        let dish_in_transit = self.pending_dish.clone();

        if let Some(prev_state) = self.prev_state {
            if prev_state == AppState::EditingDish {
                self.db.dishes[self.db_cursor].ingredients[self.edit_cursor].category =
                    chosen_category;
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

    fn find_category(&mut self, input: String) -> Category {
        let mut look_up = String::new();

        for char in input.chars() {
            if char == ' ' {
                continue;
            } else {
                look_up.push(char);
            }
        }

        if let Some(c) = self.category_db.get(&look_up.to_lowercase()) {
            return c.to_owned();
        } else {
            Category::Misc
        }
    }
}

pub fn uppercase_words(data: &str) -> String {
    // Uppercase first letter in string, and letters after spaces.
    let mut result = String::new();
    let mut first = true;
    for value in data.chars() {
        if first {
            result.push(value.to_uppercase().nth(0).unwrap());
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

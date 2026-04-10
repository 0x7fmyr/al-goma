use crate::items::{self, Category, Database, Dish};
use crate::list;
use crate::ui::Cursor;
use crate::ui::update_scroll;
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
}

#[derive(Debug)]
pub struct App {
    pub current_dish_list: Option<Vec<Dish>>,
    pub shopping_list: Vec<Ingredient>,
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
}

impl App {
    pub fn init() -> Self {
        App {
            current_dish_list: list::load(),
            shopping_list: list::load_shopping_list_config(),
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
            category_db: items::ingredient_category_db(),
            state: AppState::Normal,
            prev_state: None,
            input: String::new(),
            pending_dish: None,
            left_window_actions: vec![
                "New List",
                "View/Edit List",
                "Add Dish to Dishtabase",
                "View/Edit Dishtabase",
                //"Upload",
            ],
        }
    }
    pub fn keyboard_input(&mut self, c: char) {
        match self.state {
            AppState::EnteringDishName
            | AppState::EnteringIngredients
            | AppState::EditingIngredient
            | AppState::EditingDishName
            | AppState::EditingAddIngredient
            | AppState::NewList => {
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
            | AppState::NewList => {
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
                    if self.current_dish_list.is_some() {
                        self.state = AppState::ReplaceList;
                        self.selected_space = Space::MainRight;
                        self.moving_focus = false;

                        return;
                    }

                    self.state = AppState::NewList;
                    self.selected_space = Space::MainRight
                } else if self.selected_space == Space::MainLeft && self.cursor == 1 {
                    self.state = AppState::ShowShoppingList;
                    self.selected_space = Space::MainRight;
                    return;
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
            AppState::ViewingDatabase => self.state = AppState::AreYouSureDelDish,
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
            update_scroll(&mut self.edit_cursor);
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
        if self.db.dishes.len() == 0 {
            return;
        }
        self.db.dishes.remove(self.db_cursor.cursor);

        self.ays_cursor = 0;

        if self.db_cursor.cursor == self.db.dishes.len() && self.db.dishes.len() > 0 {
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
                        && pending_dish.ingredients.len() > 0
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
                    if self.db_cursor.cursor == self.shopping_list.len()
                        || self.db_cursor.cursor > self.shopping_list.len()
                    {
                        if !self.shopping_list.is_empty() {
                            self.db_cursor.cursor = self.shopping_list.len() - 1;
                        }
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

        if let Some(prev_state) = self.prev_state {
            if prev_state == AppState::EditingDish {
                self.db.dishes[self.db_cursor.cursor].ingredients[self.edit_cursor.cursor]
                    .category = chosen_category;
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

use crate::items::{Database, Dish};
use crate::{db, items::Ingredient};

#[derive(Debug, PartialEq)]
pub enum Space {
    MainLeft,
    MainRight,
}

#[derive(Debug, PartialEq)]
pub enum AppState {
    Normal,
    EnteringDishName,
    EnteringIngredients,
    PickingCategory,
    ViewingDatabase,
    EditingItem,
    EditingIngredient,
}

#[derive(Debug)]
pub struct App {
    pub list: Option<Vec<String>>,
    pub cursor: usize,
    pub db_cursor: usize,
    pub edit_cursor: usize,
    pub moving_focus: bool,
    pub selected_space: Space,
    pub left_window_actions: Vec<&'static str>,
    pub db: Database,
    pub state: AppState,
    pub input: String,
    pub pending_dish: Option<Dish>,
}

impl App {
    pub fn init() -> Self {
        App {
            list: None,
            cursor: 0,
            db_cursor: 0,
            edit_cursor: 0,
            moving_focus: false,
            selected_space: Space::MainLeft,
            db: db::load(),
            state: AppState::Normal,
            input: String::new(),
            pending_dish: None,
            left_window_actions: vec![
                "New List",
                "Edit List",
                "Add Dish",
                "Add Dish to Database",
                "View/Edit Database",
                "Upload",
            ],
        }
    }
    pub fn keyboard_input(&mut self, c: char) {
        match self.state {
            AppState::EnteringDishName
            | AppState::EnteringIngredients
            | AppState::EditingIngredient => {
                self.input.push(c);
            }
            _ => {}
        }
    }

    pub fn backspace(&mut self) {
        match self.state {
            AppState::EnteringDishName
            | AppState::EnteringIngredients
            | AppState::EditingIngredient => {
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
                    self.state = AppState::EditingItem;
                }
                AppState::EditingItem => {
                    self.state = AppState::EditingIngredient;
                    self.pending_dish = Some(self.db.dishes[self.db_cursor].to_owned());
                }
                AppState::EditingIngredient => self.edit_ingredient(),
                _ => {}
            }
        }
    }

    pub fn handle_esc(&mut self) {
        if matches!(self.state, AppState::EditingItem) {
            self.state = AppState::ViewingDatabase;
            self.edit_cursor = 0;

        } else if matches!(self.state, AppState::EditingIngredient) {
            self.state = AppState::EditingItem;
            self.pending_dish = None;
            self.input.clear();
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
            AppState::EnteringIngredients => self.delete_ingredient(),
            _ => {}
        }
    }

    pub fn push_dish_to_db(&mut self) {
        if let Some(dish) = self.pending_dish.take() {
            self.db.dishes.push(dish);
            db::save(&self.db);
        }

        self.state = AppState::EnteringDishName;
        self.input.clear();
    }

    fn edit_ingredient(&mut self) {
        if let Some(pending_dish) = self.pending_dish.as_mut() {
            let input = uppercase_words(&self.input.clone());

            pending_dish.ingredients[self.edit_cursor].name.clear();
            pending_dish.ingredients[self.edit_cursor].name = input;

            self.db.dishes[self.db_cursor] = pending_dish.clone();

            self.input.clear();

            db::save(&self.db);

            self.pending_dish = None;

            self.state = AppState::EditingItem
        }
    }

    fn delete_ingredient(&mut self) {
        if let Some(pending_dish) = self.pending_dish.as_mut() {
            pending_dish.ingredients.pop();
        }
    }

    fn confirm_ingredient(&mut self) {
        let input = uppercase_words(&self.input.clone());

        if input.is_empty() {
            return;
        }

        if let Some(dish) = self.pending_dish.as_mut() {
            dish.ingredients.push(Ingredient {
                name: input,
                frozen: false,
                category: crate::items::Category::Misc,
            });
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
}

fn uppercase_words(data: &str) -> String {
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

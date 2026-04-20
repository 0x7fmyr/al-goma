use crate::app::{App, AppState};
use crate::items::{Database, Dish, Ingredient};
use std::collections::HashSet;
use std::fs;

use rand::Rng;
use serde::{Deserialize, Serialize};

impl App {
    pub fn generate_list(&mut self) {
        if self.db.dishes.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();
        for c in self.input.chars() {
            if !c.is_numeric() {
                return;
            }
        }
        let mut number_of_dishes: usize;
        if !self.input.is_empty() {
            number_of_dishes = self.input.parse::<usize>().unwrap();
        } else {
            self.state = AppState::Normal;
            return;
        }

        let mut found_dishes: Vec<Dish> = Vec::new();
        let mut i = 0;

        if number_of_dishes > self.db.dishes.len() {
            number_of_dishes = self.db.dishes.len();
        }

        while i < number_of_dishes {
            let d = rng.gen_range(0..self.db.dishes.len());
            let dish = self.db.dishes[d].clone();

            if found_dishes.contains(&dish) {
                continue;
            }

            found_dishes.push(dish);

            i += 1
        }

        self.current_dish_list.replace(found_dishes); // = Some(found_dishes);

        make_shopping_list(self.current_dish_list.clone(), &mut self.shopping_list);

        save_list(self.current_dish_list.clone());

        save_shopping_list_config(self.shopping_list.clone());

        self.input.clear();
        self.state = AppState::ShowGeneratedList
    }

    pub fn generate_new_dish(&mut self) {
        
        
        let mut rng = rand::thread_rng();
        let selected_dish = self.edit_cursor.cursor;

        loop {
            let i = rng.gen_range(0..self.db.dishes.len());
            let rand_dish = self.db.dishes[i].clone();

            if let Some(list) = self.current_dish_list.as_mut() {
                
                if list.len() == self.db.dishes.len(){
                    return;
                }
                
                if list[selected_dish].name == rand_dish.name {
                    continue;
                }

                if list.contains(&rand_dish) {
                    continue;
                }

                list[selected_dish] = self.db.dishes[i].clone();
                break;
            }
        }
    }
}

fn make_shopping_list(list: Option<Vec<Dish>>, shopping_list: &mut Vec<Ingredient>) {
    shopping_list.clear();
    if let Some(list) = list {
        let mut seen = HashSet::new();
        for d in list.iter() {
            for i in d.ingredients.clone() {
                if seen.insert(i.name.clone()) {
                    shopping_list.push(i.clone());
                }
            }
        }
        shopping_list.sort_by_key(|c| c.category);
    }
}

pub fn save_list(list: Option<Vec<Dish>>) {
    if let Some(save_list) = list {
        let save_file = Database { dishes: save_list };
        let contents = toml::to_string(&save_file).expect("failed to serialize...");
        fs::create_dir_all(".config/").expect("failed to make dir: .config");
        fs::write(".config/list.toml", contents).expect("failed to write file...")
    }
}

#[derive(Serialize, Deserialize)]
pub struct ShListSaveFile {
    pub items: Vec<Ingredient>,
}

pub fn save_shopping_list_config(shopping_list: Vec<Ingredient>) {
    let save_file = ShListSaveFile {
        items: shopping_list,
    };

    let contents = toml::to_string(&save_file).expect("failed to serialize...");

    fs::create_dir_all(".config/").expect("failed to make dir: .config");
    fs::write(".config/sh_list.toml", contents).expect("failed to write file...")
}

pub fn load_shopping_list_config() -> Vec<Ingredient> {
    let contents = match fs::read_to_string(".config/sh_list.toml") {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    let list_load: ShListSaveFile = toml::from_str(&contents).expect("sh_list.toml is fucked!");

    list_load.items
}

pub fn load() -> Option<Vec<Dish>> {
    let contents = match fs::read_to_string(".config/list.toml") {
        Ok(s) => s,
        Err(_) => return None,
    };
    let list_db: Database = toml::from_str(&contents).expect("list.toml is fucked!");
    Some(list_db.dishes)
}

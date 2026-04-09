use crate::app::{App, AppState, Space};
use crate::items::{Database, Dish, Ingredient};
use std::collections::HashSet;
use std::fs;

use rand::Rng;

impl App {
    pub fn generate_list(&mut self) {
        let mut rng = rand::thread_rng();
        for c in self.input.chars() {
            if !c.is_numeric() {
                return;
            }
        }
        let mut number_of_dishes = self.input.parse::<usize>().unwrap();
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

        self.list.replace(found_dishes); // = Some(found_dishes);

        make_shopping_list(self.list.clone(), &mut self.shopping_list);

        let save_file = Database {
            dishes: self.list.to_owned().unwrap(),
        };
        save_list(&save_file);
        self.input.clear();
        self.state = AppState::ShowGeneratedList
    }

    pub fn generate_new_dish(&mut self) {
        let mut rng = rand::thread_rng();
        let selected_dish = self.edit_cursor.cursor;

        loop {
            let i = rng.gen_range(0..self.db.dishes.len());
            let rand_dish = self.db.dishes[i].clone();

            if let Some(list) = self.list.as_mut() {
                if list[selected_dish].name == rand_dish.name {
                    continue;
                }

                for i in list.iter() {
                    if *i == rand_dish {
                        continue;
                    }
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

pub fn save_list(list: &Database) {
    let contents = toml::to_string(list).expect("failed to serialize...");
    fs::write("list.toml", contents).expect("failed to write file...")
}

pub fn init_shopping_list(list: Option<Vec<Dish>>) -> Vec<Ingredient> {
    let mut shopping_list: Vec<Ingredient> = Vec::new();
    if let Some(dishes) = list {
        for i in dishes.iter() {
            for x in i.ingredients.clone() {
                shopping_list.push(x);
            }
        }
    }
    shopping_list
}

pub fn load() -> Option<Vec<Dish>> {
    let contents = match fs::read_to_string("list.toml") {
        Ok(s) => s,
        Err(_) => return None,
    };
    let list_db: Database = toml::from_str(&contents).expect("list.toml is fucked!");
    Some(list_db.dishes)
}

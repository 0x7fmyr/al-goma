use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Category {
    Dairy,
    DryGoods,
    Spices,
    Vegtables,
    Fruit,
    Protein,
    Misc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dish {
    pub name: String,
    pub ingredients: Vec<Ingredient>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub category: Category,
    pub frozen: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub dishes: Vec<Dish>,
}

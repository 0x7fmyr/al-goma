use crate::{
    app::{App, AppState, Space},
    items,
};

impl App {
    pub fn move_focus_left(&mut self) {
        if self.state == AppState::AreYouSureDelDish {
            self.move_cursor_up();
            return;
        }

        if self.selected_space == Space::MainRight {
            self.selected_space = Space::MainLeft;
            //self.state = AppState::MovingFocus
            self.moving_focus = true
        }
    }

    pub fn move_focus_right(&mut self) {
        if self.state == AppState::AreYouSureDelDish {
            self.move_cursor_down();
            return;
        }

        if self.selected_space == Space::MainLeft {
            self.selected_space = Space::MainRight;
            self.moving_focus = false
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.state == AppState::AreYouSureDelDish {
            self.del_cursor = 1;
            return;
        }

        match self.selected_space {
            Space::MainLeft => {
                if self.cursor == self.left_window_actions.len() {
                    return;
                }
                if self.cursor < self.left_window_actions.len() - 1 {
                    self.cursor += 1
                }
            }
            Space::MainRight => match self.state {
                AppState::ViewingDatabase => {
                    if self.db.dishes.is_empty() {
                        return;
                    }

                    if self.db_cursor < self.db.dishes.len() - 1 {
                        self.db_cursor += 1
                    }
                }
                AppState::EditingDish => {
                    if self.db.dishes[self.db_cursor].ingredients.is_empty() {
                        return;
                    }

                    if self.edit_cursor < self.db.dishes[self.db_cursor].ingredients.len() - 1 {
                        self.edit_cursor += 1
                    }
                }
                AppState::PickingCategory => {
                    if self.picking_cursor == 6 {
                        return;
                    }

                    self.picking_cursor += 1
                }
                _ => {}
            },
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.state == AppState::AreYouSureDelDish {
            self.del_cursor = 0;
            return;
        }

        match self.selected_space {
            Space::MainLeft => {
                if self.cursor == 0 {
                    return;
                }

                self.cursor -= 1;
            }
            Space::MainRight => match self.state {
                AppState::ViewingDatabase => {
                    if self.db.dishes.is_empty() {
                        return;
                    }
                    if self.db_cursor > 0 {
                        self.db_cursor -= 1
                    }
                }
                AppState::EditingDish => {
                    if self.db.dishes[self.db_cursor].ingredients.is_empty() {
                        return;
                    }
                    if self.edit_cursor > 0 {
                        self.edit_cursor -= 1
                    }
                }
                AppState::PickingCategory => {
                    if self.picking_cursor == 0 {
                        return;
                    }
                    self.picking_cursor -= 1
                }
                _ => {}
            },
        }
    }
}
pub fn get_category_name(c: items::Category) -> String {
    match c {
        items::Category::Dairy => String::from("Mejeri"),
        items::Category::DryGoods => String::from("Skafferi/Torr varor"),
        items::Category::Spices => String::from("Kryddor"),
        items::Category::Vegtables => String::from("Grönsaker"),
        items::Category::Fruit => String::from("Frukt"),
        items::Category::Protein => String::from("Protein"),
        items::Category::Misc => String::from("Annat"),
    }
}

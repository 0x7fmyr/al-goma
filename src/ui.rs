use crate::{
    app::{App, AppState, Space},
    items,
    locale::UiText,
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Cursor {
    pub cursor: usize,
    pub scroll: usize,
    pub visable_lines: usize,
}

impl App {
    pub fn move_focus_left(&mut self) {
        if self.state == AppState::AreYouSureDelDish || self.state == AppState::ReplaceList {
            self.move_cursor_up();
            return;
        }

        if self.selected_space == Space::MainRight {
            self.selected_space = Space::MainLeft;
            self.moving_focus = true
        }
    }

    pub fn move_focus_right(&mut self) {
        if self.state == AppState::AreYouSureDelDish || self.state == AppState::ReplaceList {
            self.move_cursor_down();
            return;
        }

        if self.selected_space == Space::MainLeft {
            self.selected_space = Space::MainRight;
            self.moving_focus = false
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.state == AppState::AreYouSureDelDish
            || self.state == AppState::ReplaceList
            || self.state == AppState::PromptPrint
        {
            self.ays_cursor = 1;
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
                AppState::ShowShoppingList => {
                    if self.shopping_list.is_empty() {
                        return;
                    }

                    if self.db_cursor.cursor < self.shopping_list.len() - 1 {
                        self.db_cursor.cursor += 1;
                        update_scroll(&mut self.db_cursor);
                    }
                }
                AppState::ViewingDatabase | AppState::AddToGeneratedList => {
                    if self.db.dishes.is_empty() {
                        return;
                    }

                    if self.db_cursor.cursor < self.db.dishes.len() - 1 {
                        self.db_cursor.cursor += 1
                    }
                }
                AppState::EditingDish => {
                    if self.db.dishes[self.db_cursor.cursor].ingredients.is_empty() {
                        return;
                    }

                    if self.edit_cursor.cursor
                        < self.db.dishes[self.db_cursor.cursor].ingredients.len() - 1
                    {
                        self.edit_cursor.cursor += 1;
                        update_scroll(&mut self.edit_cursor);
                    }
                }
                AppState::PickingCategory => {
                    if self.picking_cursor == 6 {
                        return;
                    }

                    self.picking_cursor += 1
                }
                AppState::ShowGeneratedList => {
                    if let Some(list) = self.current_dish_list.as_ref()
                        && self.edit_cursor.cursor < list.len() - 1
                    {
                        self.edit_cursor.cursor += 1;
                    }
                }
                _ => {}
            },
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.state == AppState::AreYouSureDelDish
            || self.state == AppState::ReplaceList
            || self.state == AppState::PromptPrint
        {
            self.ays_cursor = 0;
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
                AppState::ViewingDatabase | AppState::ShowShoppingList | AppState::AddToGeneratedList => {
                    if self.shopping_list.is_empty()
                        && matches!(self.state, AppState::ShowShoppingList)
                    {
                        return;
                    }

                    if self.db_cursor.cursor > 0 {
                        self.db_cursor.cursor -= 1;
                        update_scroll(&mut self.db_cursor);
                    }
                }
                AppState::EditingDish => {
                    if self.db.dishes[self.db_cursor.cursor].ingredients.is_empty() {
                        return;
                    }
                    if self.edit_cursor.cursor > 0 {
                        self.edit_cursor.cursor -= 1;
                        update_scroll(&mut self.edit_cursor);
                    }
                }
                AppState::PickingCategory => {
                    if self.picking_cursor == 0 {
                        return;
                    }
                    self.picking_cursor -= 1
                }
                AppState::ShowGeneratedList
                    if self.edit_cursor.cursor > 0 => {
                        self.edit_cursor.cursor -= 1;
                    }
                _ => {}
            },
        }
    }

    pub fn get_category_name(&self, c: items::Category) -> String {
        match c {
            items::Category::Dairy => self.text_get(UiText::Dairy),
            items::Category::Pantry => self.text_get(UiText::Pantry),
            items::Category::Spices => self.text_get(UiText::Spices),
            items::Category::Vegetables => self.text_get(UiText::Vegetables),
            items::Category::Fruit => self.text_get(UiText::Fruit),
            items::Category::Protein => self.text_get(UiText::Protein),
            items::Category::Misc => self.text_get(UiText::Misc),
        }
    }
}

pub fn update_scroll(input: &mut Cursor) {
    if input.cursor < input.scroll {
        input.scroll = input.cursor;
    } else if input.cursor >= input.scroll + input.visable_lines {
        input.scroll = input.cursor - input.visable_lines + 1;
    }
}

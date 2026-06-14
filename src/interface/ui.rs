use crate::{
    app::{App, AppState, Space},
    dishtabase,
    interface::locale::UiText,
    lists,
};

use chrono::Utc;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Cursor {
    pub cursor: usize,
    pub scroll: usize,
    pub visable_lines: usize,
}

impl App {
    // Main menu open
    pub fn open_new_list(&mut self) {
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
    }

    pub fn open_view_edit_list(&mut self) {
        self.state = AppState::ShowShoppingList;
        self.selected_space = Space::MainRight;
    }

    pub fn open_add_dish_to_dishtabase(&mut self) {
        self.state = AppState::EnteringDishName;
        self.selected_space = Space::MainRight
    }

    pub fn open_view_edit_dishtabase(&mut self) {
        dishtabase::db::load();
        self.state = AppState::ViewingDatabase;
        self.selected_space = Space::MainRight;
    }

    pub fn open_upload(&mut self) {
        match lists::upload::does_token_exist() {
            Ok(true) => {
                self.state = AppState::UploadMenu;
                self.input = format!("Shopping List {}", Utc::now().date_naive());
                self.default_upload_name = true;
            }
            Ok(false) => self.state = AppState::UploadFirstLogin,
            Err(s) => {
                self.state = AppState::Error;
                self.err_msg = Some(s)
            }
        }
        self.selected_space = Space::MainRight
    }

    // Focus
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
        if self.state == AppState::Normal {
            return;
        }

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
                if self.cursor == self.left_window_actions.len() - 1 {
                    self.cursor = 0;
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

                    update_scroll(&mut self.db_cursor);
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
                    self.cursor = 4;
                    return;
                }

                self.cursor -= 1;
            }
            Space::MainRight => match self.state {
                AppState::ViewingDatabase
                | AppState::ShowShoppingList
                | AppState::AddToGeneratedList => {
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
                AppState::ShowGeneratedList if self.edit_cursor.cursor > 0 => {
                    self.edit_cursor.cursor -= 1;
                }
                _ => {}
            },
        }
    }

    pub fn get_category_name(&self, c: dishtabase::items::Category) -> String {
        match c {
            dishtabase::items::Category::Dairy => self.text_get(UiText::Dairy),
            dishtabase::items::Category::Pantry => self.text_get(UiText::Pantry),
            dishtabase::items::Category::Spices => self.text_get(UiText::Spices),
            dishtabase::items::Category::Vegetables => self.text_get(UiText::Vegetables),
            dishtabase::items::Category::Fruit => self.text_get(UiText::Fruit),
            dishtabase::items::Category::Protein => self.text_get(UiText::Protein),
            dishtabase::items::Category::Misc => self.text_get(UiText::Misc),
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

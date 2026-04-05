use crate::app::{App, AppState, Space};

impl App {
    pub fn move_focus_left(&mut self) {
        if self.selected_space == Space::MainRight {
            self.selected_space = Space::MainLeft;
            //self.state = AppState::MovingFocus
            self.moving_focus = true
        }
    }

    pub fn move_focus_right(&mut self) {
        if self.selected_space == Space::MainLeft {
            self.selected_space = Space::MainRight;
            self.moving_focus = false
        }
    }

    pub fn move_cursor_down(&mut self) {
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
                AppState::EditingItem => {
                    if self.db.dishes[self.db_cursor].ingredients.is_empty() {
                        return;
                    }

                    if self.edit_cursor < self.db.dishes[self.db_cursor].ingredients.len() - 1 {
                        self.edit_cursor += 1
                    }
                }
                _ => {}
            },
        }
    }

    pub fn move_cursor_up(&mut self) {
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
                AppState::EditingItem => {
                    if self.db.dishes[self.db_cursor].ingredients.is_empty() {
                        return;
                    }
                    if self.edit_cursor > 0 {
                        self.edit_cursor -= 1
                    }
                }
                _ => {}
            },
        }
    }
}

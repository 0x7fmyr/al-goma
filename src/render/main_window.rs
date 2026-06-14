use ratatui::{
    Frame,
    layout::Rect,
    layout::{Alignment, Constraint, Layout, Margin},
    prelude::Direction,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType::Rounded, Borders, Paragraph},
};

use super::{pop, render_db, render_upload};

use crate::{
    app::{self, AppState, Space},
    interface::locale::UiText,
    render::{self, new_list},
};

pub fn left(window: &mut Frame, rect: Rect, app: &mut app::App) {
    window.render_widget(
        Paragraph::default().block(
            Block::bordered()
                .border_style(Style::default().fg(Color::DarkGray).dim())
                .border_type(Rounded)
                .borders(Borders::ALL),
        ),
        rect,
    );

    let action_list_window = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1)])
        .split(rect.inner(Margin {
            horizontal: 2,
            vertical: 2,
        }));

    let mut line: Vec<Line> = Vec::new();

    for (i, a) in app.left_window_actions.iter().enumerate() {
        if i == app.cursor && app.selected_space == Space::MainLeft {
            line.push(Line::from(Span::styled(
                a.to_string(),
                Style::new()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            )));
        } else {
            line.push(Line::from(Span::styled(
                a.to_string(),
                Style::new().fg(Color::DarkGray),
            )));
        }
        line.push(Line::from(""));
    }
    window.render_widget(
        Paragraph::new(line).alignment(Alignment::Center),
        action_list_window[0],
    );
}

pub fn right(window: &mut Frame, rect: Rect, app: &mut app::App) {
    let prev_state = app.prev_state.unwrap_or(AppState::Normal);

    //ToolTips

    let tooltip: String = match app.state {
        AppState::EnteringDishName | AppState::EnteringIngredients => {
            app.text_get(UiText::TTInputBox)
        }
        AppState::ViewingDatabase => app.text_get(UiText::TTViewingDb),
        AppState::EditingDish => app.text_get(UiText::TTEditingDish),
        AppState::EditingIngredient | AppState::EditingDishName => {
            app.text_get(UiText::TTEditingIngDName)
        }

        AppState::PickingCategory | AppState::AreYouSureDelDish => app.text_get(UiText::TTPopUp),
        AppState::ShowGeneratedList => app.text_get(UiText::TTShowGenList),
        AppState::ShowShoppingList => app.text_get(UiText::TTShowShoppingList),
        AppState::PromptPrint => app.text_get(UiText::TTPromtPrint),
        _ => "".to_string(),
    };

    let main_block = Block::bordered()
        .border_style(Style::default().fg(Color::DarkGray).dim())
        .border_type(Rounded)
        .borders(Borders::ALL)
        .title(tooltip)
        .title_position(ratatui::widgets::block::Position::Bottom);

    window.render_widget(Paragraph::default().block(main_block), rect);

    //New List

    if matches!(app.state, AppState::NewList | AppState::ReplaceList) {
        if matches!(app.state, AppState::ReplaceList) {
            let msg = vec![
                Line::from(app.text_get(UiText::GeneratingReplaceOld1)),
                Line::from(app.text_get(UiText::GeneratingReplaceOld2)),
                Line::from(app.text_get(UiText::GeneratingReplaceOld3)),
            ];

            render::pop::are_you_sure(window, rect, app, msg, (11, 45));
        } else {
            render::new_list::new_list(window, rect, app, (10, 45));
        }
    }

    if matches!(
        app.state,
        AppState::ShowGeneratedList | AppState::AddToGeneratedList
    ) {
        render::new_list::show_generated_list(window, rect, app, (20, 40));

        if matches!(app.state, AppState::AddToGeneratedList) {
            pop::add_to_generated_list(window, rect, app, (14, 50));
        }
    }

    //View/Edit List

    if matches!(
        app.state,
        AppState::ShowShoppingList | AppState::AddToShoppingList | AppState::PromptPrint
    ) || app.state == AppState::PickingCategory && prev_state == AppState::AddToShoppingList
    {
        new_list::show_generated_list_ingredients(window, rect, app);

        if matches!(app.state, AppState::PickingCategory) {
            let i_name = app.shopping_list.last().unwrap().name.clone();
            pop::pick_category(window, rect, app, prev_state, i_name, (14, 38));
        }

        if matches!(app.state, AppState::PromptPrint) {
            pop::print_txt_options(window, rect, app, (10, 38));
        }
    }

    //Add Dish to Dishtabase

    if matches!(
        app.state,
        AppState::EnteringDishName | AppState::EnteringIngredients
    ) || app.state == AppState::PickingCategory && prev_state == AppState::EnteringIngredients
    {
        render_db::add_dish(window, rect, app, prev_state);
    }

    //View/Edit Dishtabase

    if matches!(
        app.state,
        AppState::ViewingDatabase
            | AppState::EditingDish
            | AppState::EditingIngredient
            | AppState::EditingDishName
            | AppState::AreYouSureDelDish
            | AppState::EditingAddIngredient
    ) || app.state == AppState::PickingCategory && prev_state == AppState::EditingDish
    {
        render_db::dish_database(window, rect, app);

        if matches!(
            app.state,
            AppState::EditingDish
                | AppState::EditingIngredient
                | AppState::EditingDishName
                | AppState::EditingAddIngredient
                | AppState::PickingCategory
        ) {
            if rect.height > 12 {
                render_db::edit_widow(window, rect, app, ((rect.height * 4) / 6, 45));
            }

            if app.state == AppState::PickingCategory {
                pop::pick_category(
                    window,
                    rect,
                    app,
                    prev_state,
                    app.pending_dish
                        .as_ref()
                        .unwrap()
                        .ingredients
                        .last()
                        .unwrap()
                        .name
                        .clone(),
                    (14, 38),
                );
            }
        }

        if app.state == AppState::AreYouSureDelDish {
            let deleting_name = app.db.dishes[app.db_cursor.cursor].name.clone();
            let msg = vec![
                Line::from(format!(
                    "{} {}",
                    app.text_get(UiText::DeletingAys1),
                    deleting_name
                )),
                Line::from(""),
                Line::from(app.text_get(UiText::DeletingAys2)),
            ];

            pop::are_you_sure(window, rect, app, msg, (11, 50));
        }
    }

    if matches!(
        app.state,
        AppState::EnteringDishName | AppState::EditingDishName
    ) {
        pop::input_box(window, rect, app, app.text_get(UiText::PPEnterDishName));
    }

    if matches!(
        app.state,
        AppState::EnteringIngredients
            | AppState::EditingIngredient
            | AppState::EditingAddIngredient
            | AppState::AddToShoppingList
    ) {
        pop::input_box(window, rect, app, app.text_get(UiText::PPEnterIngredient));
    }

    if matches!(
        app.state,
        AppState::UploadFirstLogin
            | AppState::UploadWaitingForLoginUrl
            | AppState::UploadLogginginWait
    ) {
        let mut msg: Vec<Line> = Vec::from([Line::from("n/a")]);

        if app.state == AppState::UploadFirstLogin {
            msg = Vec::from([
                Line::from(Span::styled(
                    app.text_get(UiText::UPFirstLogin1),
                    Style::new().add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(""),
                Line::from(app.text_get(UiText::UPFirstLogin2)),
                Line::from(app.text_get(UiText::UPFirstLogin3)),
                Line::from(app.text_get(UiText::UPFirstLogin4)),
            ]);
        }

        if app.state == AppState::UploadWaitingForLoginUrl {
            msg = Vec::from([Line::from(Span::styled(
                app.text_get(UiText::UPWaiting4Url),
                Style::new().add_modifier(Modifier::BOLD),
            ))]);
        }
        if app.state == AppState::UploadLogginginWait {
            msg = Vec::from([Line::from(Span::styled(
                app.text_get(UiText::UPWaiting4Google),
                Style::new().add_modifier(Modifier::BOLD),
            ))]);
        }
        render_upload::login_popup(window, rect, msg);
    }

    // Upload

    if matches!(
        app.state,
        AppState::UploadMenu | AppState::Uploading | AppState::UploadDone
    ) {
        render_upload::upload_menu(window, rect, app);
    }

    // Error Popup

    if app.state == AppState::Error {
        let error_msg = app.err_msg.clone().unwrap();
        pop::err_pop_up(window, rect, error_msg);
    }
}

pub fn center_rect(rect: Rect, width: u16, height: u16) -> Rect {
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(width),
            Constraint::Fill(1),
        ])
        .split(rect);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(height),
            Constraint::Fill(1),
        ])
        .split(v[1])[1]
}

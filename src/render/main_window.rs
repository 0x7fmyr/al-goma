use ratatui::layout::{Alignment, Constraint, Layout, Margin};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::Clear;
use ratatui::widgets::{Block, BorderType::Rounded, Borders, Paragraph};
use ratatui::{Frame, layout::Rect};

use super::db;
use super::pop;
use crate::app::{self, AppState, Space};
use crate::locale::UiText;
use crate::render::{self, new_list};

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
    let mut tooltip = String::from("");
    let mut prev_state = AppState::Normal;

    if let Some(state) = app.prev_state {
        prev_state = state
    }

    match app.state {
        AppState::EnteringDishName | AppState::EnteringIngredients => {
            tooltip = app.text_get(UiText::TTInputBox)
        }
        AppState::ViewingDatabase => tooltip = app.text_get(UiText::TTViewingDb),
        AppState::EditingDish => tooltip = app.text_get(UiText::TTEditingDish),
        AppState::EditingIngredient | AppState::EditingDishName => {
            tooltip = app.text_get(UiText::TTEditingIngDName)
        }

        AppState::PickingCategory | AppState::AreYouSureDelDish => {
            tooltip = app.text_get(UiText::TTPopUp)
        }
        AppState::ShowGeneratedList => tooltip = app.text_get(UiText::TTShowGenList),
        AppState::ShowShoppingList => tooltip = app.text_get(UiText::TTShowShoppingList),
        AppState::PromptPrint => tooltip = app.text_get(UiText::TTPromtPrint),
        _ => {}
    }

    let main_block = Block::bordered()
        .border_style(Style::default().fg(Color::DarkGray).dim())
        .border_type(Rounded)
        .borders(Borders::ALL)
        .title(tooltip)
        .title_position(ratatui::widgets::block::Position::Bottom);

    window.render_widget(Paragraph::default().block(main_block), rect);

    if matches!(app.state, AppState::NewList) || matches!(app.state, AppState::ReplaceList) {
        let mut w: u16 = 40;
        let mut h: u16 = 10;
        let center_y = rect.y + (rect.height / 2) - (h / 2);
        let center_x = rect.x + (rect.width / 2) - (w / 2);

        if matches!(app.state, AppState::ReplaceList) {
            w += 5;
            h += 1;
            let msg = vec![
                Line::from(app.text_get(UiText::GeneratingReplaceOld1)),
                Line::from(app.text_get(UiText::GeneratingReplaceOld2)),
                Line::from(app.text_get(UiText::GeneratingReplaceOld3)),
            ];
            window.render_widget(
                Clear,
                Rect {
                    x: center_x,
                    y: center_y,
                    width: w,
                    height: h,
                },
            );

            render::pop::are_you_sure(
                window,
                Rect {
                    x: center_x,
                    y: center_y,
                    width: w,
                    height: h,
                },
                app,
                msg,
            );
        } else {
            window.render_widget(
                Clear,
                Rect {
                    x: center_x,
                    y: center_y,
                    width: w,
                    height: h,
                },
            );

            render::new_list::new_list(
                window,
                Rect {
                    x: center_x,
                    y: center_y,
                    width: w,
                    height: h,
                },
                app,
            );
        }
    }

    if matches!(app.state, AppState::ShowGeneratedList) {
        let w: u16 = 40;
        let h: u16 = 20;
        let center_y = rect.y + (rect.height / 2) - (h / 2);
        let center_x = rect.x + (rect.width / 2) - (w / 2);

        window.render_widget(
            Clear,
            Rect {
                x: center_x,
                y: center_y,
                width: w,
                height: h,
            },
        );

        render::new_list::show_generated_list(
            window,
            Rect {
                x: center_x,
                y: center_y,
                width: w,
                height: h,
            },
            app,
        );
    }

    if matches!(app.state, AppState::ShowShoppingList)
        || matches!(app.state, AppState::AddToShoppingList)
        || (matches!(app.state, AppState::PickingCategory)
            && prev_state == AppState::AddToShoppingList)
        || matches!(app.state, AppState::PromptPrint)
    {
        new_list::show_generated_list_ingredients(window, rect, app);

        if matches!(app.state, AppState::PickingCategory) {
            let mut input_w: u16 = 38;
            let mut input_h: u16 = 14;
            let center_y: u16;
            let center_x: u16;

            if rect.height >= 40 || rect.width >= 40 {
                center_y = rect.y + (rect.height / 2) - (input_h / 2);
                center_x = rect.x + (rect.width / 2) - (input_w / 2);
            } else {
                center_y = 0;
                center_x = 0;
                input_h = 0;
                input_w = 0;
            }

            window.render_widget(
                Clear,
                Rect {
                    x: center_x,
                    y: center_y,
                    width: input_w,
                    height: input_h,
                },
            );

            let i_name = app.shopping_list.last().unwrap().name.clone();

            pop::pick_category(
                window,
                Rect {
                    x: center_x,
                    y: center_y,
                    width: input_w,
                    height: input_h,
                },
                app,
                prev_state,
                i_name,
            );
        }

        if matches!(app.state, AppState::PromptPrint) {
            let mut input_w: u16 = 38;
            let mut input_h: u16 = 10;
            let center_y: u16;
            let center_x: u16;

            if rect.height >= 14 || rect.width >= 38 {
                center_y = rect.y + (rect.height / 2) - (input_h / 2);
                center_x = rect.x + (rect.width / 2) - (input_w / 2);
            } else {
                center_y = 0;
                center_x = 0;
                input_h = 0;
                input_w = 0;
            }
            // todo: work!

            window.render_widget(
                Clear,
                Rect {
                    x: center_x,
                    y: center_y,
                    width: input_w,
                    height: input_h,
                },
            );

            pop::print_txt_options(
                window,
                Rect {
                    x: center_x,
                    y: center_y,
                    width: input_w,
                    height: input_h,
                },
                app,
            );
        }
    }

    if matches!(app.state, AppState::EnteringDishName)
        || matches!(app.state, AppState::EnteringIngredients)
        || (matches!(app.state, AppState::PickingCategory)
            && prev_state == AppState::EnteringIngredients)
    {
        db::add_dish(window, rect, app, prev_state);
    }

    let input_w: u16 = rect.width / 2 + rect.width / 3;
    let input_h: u16 = 3;
    let center_x = rect.x + (rect.width / 2) - (input_w / 2);

    let center_y: u16 = if rect.height > 3 {
        rect.y + (rect.height - 8 / 2) - (input_h / 2)
    } else {
        2
    };

    if matches!(app.state, AppState::ViewingDatabase)
        || matches!(app.state, AppState::EditingDish)
        || matches!(app.state, AppState::EditingIngredient)
        || matches!(app.state, AppState::EditingDishName)
        || matches!(app.state, AppState::AreYouSureDelDish)
        || matches!(app.state, AppState::EditingAddIngredient)
        || (matches!(app.state, AppState::PickingCategory) && prev_state == AppState::EditingDish)
    {
        db::dish_database(window, rect, app);

        if matches!(app.state, AppState::EditingDish)
            || matches!(app.state, AppState::EditingIngredient)
            || matches!(app.state, AppState::EditingDishName)
            || matches!(app.state, AppState::EditingAddIngredient)
            || matches!(app.state, AppState::PickingCategory)
        {
            let mut input_w: u16 = 40;
            let input_h: u16 = (rect.height * 4) / 6;
            let center_y: u16;
            let center_x: u16;

            if rect.height > (rect.height * 3) / 4 && rect.width > 30 {
                center_y = rect.y + (rect.height / 2) - (input_h / 2);
                center_x = rect.x + (rect.width / 2) - (input_w / 2);
            } else {
                input_w = 2;
                center_y = 2;
                center_x = 2
            }

            window.render_widget(
                Clear,
                Rect {
                    x: center_x,
                    y: center_y,
                    width: input_w,
                    height: input_h,
                },
            );

            if rect.height > 12 {
                db::edit_widow(
                    window,
                    Rect {
                        x: center_x,
                        y: center_y,
                        width: input_w,
                        height: input_h,
                    },
                    app,
                );
            }

            if matches!(app.state, AppState::PickingCategory) {
                let mut input_w: u16 = 38;
                let mut input_h: u16 = 14;
                let center_y: u16;
                let center_x: u16;

                if rect.height >= 40 || rect.width >= 40 {
                    center_y = rect.y + (rect.height / 2) - (input_h / 2);
                    center_x = rect.x + (rect.width / 2) - (input_w / 2);
                } else {
                    center_y = 0;
                    center_x = 0;
                    input_h = 0;
                    input_w = 0;
                }

                window.render_widget(
                    Clear,
                    Rect {
                        x: center_x,
                        y: center_y,
                        width: input_w,
                        height: input_h,
                    },
                );

                pop::pick_category(
                    window,
                    Rect {
                        x: center_x,
                        y: center_y,
                        width: input_w,
                        height: input_h,
                    },
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
                );
            }
        }

        if matches!(app.state, AppState::AreYouSureDelDish) {
            let mut input_w: u16 = 50;
            let mut input_h: u16 = 11;
            let center_y: u16;
            let center_x: u16;

            if rect.height >= 40 || rect.width >= 40 {
                center_y = rect.y + (rect.height / 2) - (input_h / 2);
                center_x = rect.x + (rect.width / 2) - (input_w / 2);
            } else {
                center_y = 0;
                center_x = 0;
                input_h = 0;
                input_w = 0;
            }

            window.render_widget(
                Clear,
                Rect {
                    x: center_x,
                    y: center_y,
                    width: input_w,
                    height: input_h,
                },
            );

            let deleting_name = app.db.dishes[app.db_cursor.cursor].name.clone();
            let msg = vec![
                Line::from(format!(
                    "{}: {}",
                    app.text_get(UiText::DeletingAys1),
                    deleting_name
                )),
                Line::from(""),
                Line::from(app.text_get(UiText::DeletingAys2)),
            ];

            pop::are_you_sure(
                window,
                Rect {
                    x: center_x,
                    y: center_y,
                    width: input_w,
                    height: input_h,
                },
                app,
                msg,
            );
        }
    }

    if matches!(app.state, AppState::EnteringDishName)
        || matches!(app.state, AppState::EditingDishName)
    {
        window.render_widget(
            Clear,
            Rect {
                x: center_x,
                y: center_y,
                width: input_w,
                height: input_h,
            },
        );

        pop::input_box(
            window,
            Rect {
                x: center_x,
                y: center_y,
                width: input_w,
                height: input_h,
            },
            app,
            app.text_get(UiText::PPEnterDishName),
        );
    }

    if matches!(app.state, AppState::EnteringIngredients)
        || matches!(app.state, AppState::EditingIngredient)
        || matches!(app.state, AppState::EditingAddIngredient)
        || matches!(app.state, AppState::AddToShoppingList)
    {
        window.render_widget(
            Clear,
            Rect {
                x: center_x,
                y: center_y,
                width: input_w,
                height: input_h,
            },
        );

        pop::input_box(
            window,
            Rect {
                x: center_x,
                y: center_y,
                width: input_w,
                height: input_h,
            },
            app,
            app.text_get(UiText::PPEnterIngredient),
        );
    }
}

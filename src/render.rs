use std::future::Pending;

use ratatui::layout::{Alignment, Constraint, Layout, Margin};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::Clear;
use ratatui::widgets::{Block, BorderType::Rounded, Borders, Paragraph};
use ratatui::{Frame, layout::Rect};

use crate::app::{self, AppState, Space};
use crate::items::Dish;

pub fn render_main_left(window: &mut Frame, rect: Rect, app: &mut app::App) {
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

pub fn render_main_right(window: &mut Frame, rect: Rect, app: &mut app::App) {
    let mut tooltip = String::from("");

    match app.state {
        AppState::EnteringDishName | AppState::EnteringIngredients => {
            tooltip = "[enter] confirm   [del] remove   [ctrl+s] save   [esc] cancel".to_string();
        }
        AppState::ViewingDatabase => {
            tooltip = "[up/down] select   [enter] edit   [esc] cancel".to_string();
        }
        AppState::EditingItem => {
            tooltip =
                "[up/down] select   [enter] edit   [ctrl+n] edit name   [esc] cancel".to_string();
        }
        AppState::EditingIngredient => {
            tooltip = "[enter] confirm   [esc] cancel".to_string();
        }
        _ => {}
    }

    let main_block = Block::bordered()
        .border_style(Style::default().fg(Color::DarkGray).dim())
        .border_type(Rounded)
        .borders(Borders::ALL)
        .title(tooltip)
        .title_position(ratatui::widgets::block::Position::Bottom);

    window.render_widget(Paragraph::default().block(main_block), rect);

    if matches!(app.state, AppState::EnteringDishName)
        || matches!(app.state, AppState::EnteringIngredients)
    {
        let list_window = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Fill(1)])
            .split(rect.inner(Margin {
                horizontal: 3,
                vertical: 1,
            }));

        if let Some(dish) = app.pending_dish.as_ref() {
            let name = Span::raw(dish.name.clone())
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::UNDERLINED);

            let mut ingredients: Vec<Line> = Vec::new();
            let mut spans: Vec<Span> = Vec::new();
            for (n, i) in dish.ingredients.iter().enumerate().clone() {
                let n = n + 1;
                spans.push(Span::styled(
                    n.to_string(),
                    Style::new()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                ));

                spans.push(Span::styled(
                    ". ",
                    Style::new()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                ));

                spans.push(Span::raw(i.name.clone()));
                ingredients.push(Line::from(spans.clone()));
                spans.clear();
            }

            window.render_widget(
                Paragraph::new(name).alignment(Alignment::Center),
                list_window[0],
            );

            window.render_widget(
                Paragraph::new(ingredients).alignment(Alignment::Left),
                list_window[1],
            );
        }
    }

    let input_w: u16 = rect.width / 2 + rect.width / 3;
    let input_h: u16 = 3;
    let center_y: u16;
    let center_x = rect.x + (rect.width / 2) - (input_w / 2);

    if rect.height > 3 {
        center_y = rect.y + (rect.height - 8 / 2) - (input_h / 2);
    } else {
        center_y = 2
    }

    if matches!(app.state, AppState::ViewingDatabase)
        || matches!(app.state, AppState::EditingItem)
        || matches!(app.state, AppState::EditingIngredient)
    {
        render_dish_database(window, rect, app);

        if matches!(app.state, AppState::EditingItem)
            || matches!(app.state, AppState::EditingIngredient)
        {
            let input_w: u16 = rect.width / 3;
            let input_h: u16 = (rect.height * 4) / 6;
            let center_y: u16;
            let center_x = rect.x + (rect.width / 2) - (input_w / 2);

            if rect.height > (rect.height * 3) / 4 {
                center_y = rect.y + (rect.height / 2) - (input_h / 2);
            } else {
                center_y = 2
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

            render_edit_widow(
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

    if matches!(app.state, AppState::EnteringDishName) {
        window.render_widget(
            Clear,
            Rect {
                x: center_x,
                y: center_y,
                width: input_w,
                height: input_h,
            },
        );

        render_name_input_box(
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

    if matches!(app.state, AppState::EnteringIngredients)
        || matches!(app.state, AppState::EditingIngredient)
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

        render_ingredient_input_box(
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
fn render_edit_widow(window: &mut Frame, rect: Rect, app: &mut app::App) {
    let edit_window = Layout::default()
        .constraints([Constraint::Fill(1)])
        .split(rect.inner(Margin {
            horizontal: 2,
            vertical: 2,
        }));

    let edit_items = Layout::default()
        .constraints([Constraint::Length(2), Constraint::Fill(1)])
        .split(edit_window[0].inner(Margin {
            horizontal: 1,
            vertical: 1,
        }));

    window.render_widget(
        Paragraph::default().alignment(Alignment::Left).block(
            Block::bordered()
                .border_style(Style::default().fg(Color::DarkGray).dim())
                .border_type(Rounded)
                .borders(Borders::ALL),
        ),
        edit_window[0],
    );
    let header = Line::from("Ingredients:")
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::UNDERLINED);

    let mut items: Vec<Span> = Vec::new();
    let mut ingredients: Vec<Line> = Vec::new();

    for (n, i) in app.db.dishes[app.db_cursor]
        .ingredients
        .iter()
        .enumerate()
    {
        let c = n;
        let n = n + 1;
        let mut t = Modifier::SLOW_BLINK;

        if app.state == AppState::EditingIngredient {
            t = Modifier::empty();
        }

        if c == app.edit_cursor {
            items.push(Span::styled(
                " > ",
                Style::new()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(t),
            ));

            items.push(Span::styled(
                n.to_string(),
                Style::new()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ));

            items.push(Span::styled(
                ". ",
                Style::new()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ));

            items.push(Span::styled(
                i.name.clone(),
                Style::new()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            items.push(Span::styled(
                n.to_string(),
                Style::new()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            ));

            items.push(Span::styled(
                ". ",
                Style::new()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            ));
            items.push(Span::raw(i.name.clone()));
        }

        ingredients.push(Line::from(items.clone()));
        items.clear();
    }

    window.render_widget(
        Paragraph::new(header).alignment(Alignment::Center),
        edit_items[0],
    );
    window.render_widget(Paragraph::new(ingredients), edit_items[1]);

    window.render_widget(
        Paragraph::default().alignment(Alignment::Left).block(
            Block::bordered()
                .border_style(Style::default().fg(Color::Blue))
                .border_type(Rounded)
                .borders(Borders::ALL)
                .title(format!(
                    "Editing Dish: {}",
                    app.db.dishes[app.db_cursor].name
                ))
                .title_alignment(Alignment::Center),
        ),
        rect,
    );
}
fn render_dish_database(window: &mut Frame, rect: Rect, app: &mut app::App) {
    let list_window = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Fill(1)])
        .split(rect.inner(Margin {
            horizontal: 3,
            vertical: 1,
        }));

    window.render_widget(
        Paragraph::new(
            Line::from("Dish Database")
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::UNDERLINED),
        )
        .alignment(Alignment::Center),
        list_window[0],
    );

    let mut spans: Vec<Span> = Vec::new();
    let mut dishes: Vec<Line> = Vec::new();

    if !app.db.dishes.is_empty() {
        for (i, d) in app.db.dishes.iter().enumerate() {
            let c = i;
            let i = i + 1;

            if c == app.db_cursor {
                let mut t = Modifier::SLOW_BLINK;
                if app.state == AppState::EditingItem || app.state == AppState::EditingIngredient {
                    t = Modifier::empty();
                }

                spans.push(Span::styled(
                    " > ",
                    Style::new()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(t),
                ));
                spans.push(Span::styled(
                    i.to_string(),
                    Style::new()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                ));

                spans.push(Span::styled(
                    ". ",
                    Style::new()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                ));

                spans.push(Span::styled(
                    d.name.clone(),
                    Style::new()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(
                    i.to_string(),
                    Style::new()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                ));

                spans.push(Span::styled(
                    ". ",
                    Style::new()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                ));

                spans.push(Span::raw(d.name.clone()));
            }

            dishes.push(Line::from(spans.clone()));
            spans.clear();
        }
    }

    window.render_widget(
        Paragraph::new(dishes).alignment(Alignment::Left),
        list_window[1],
    );
}

fn render_name_input_box(window: &mut Frame, rect: Rect, app: &mut app::App) {
    let input = vec![
        Span::raw(" "),
        Span::raw(app.input.clone()),
        Span::styled(
            "_",
            Style::new()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::SLOW_BLINK)
                .add_modifier(Modifier::BOLD),
        ),
    ];

    window.render_widget(
        Paragraph::new(Line::from(input))
            .alignment(Alignment::Left)
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(Color::Blue).dim())
                    .border_type(Rounded)
                    .borders(Borders::ALL)
                    .title("Enter the Dish Name")
                    .title_alignment(Alignment::Center),
            ),
        rect,
    );
}

fn render_ingredient_input_box(window: &mut Frame, rect: Rect, app: &mut app::App) {
    let input = vec![
        Span::raw(" "),
        Span::raw(app.input.clone()),
        Span::styled(
            "_",
            Style::new()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::SLOW_BLINK)
                .add_modifier(Modifier::BOLD),
        ),
    ];

    window.render_widget(
        Paragraph::new(Line::from(input))
            .alignment(Alignment::Left)
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(Color::Blue).dim())
                    .border_type(Rounded)
                    .borders(Borders::ALL)
                    .title("Enter Ingredient")
                    .title_alignment(Alignment::Center),
            ),
        rect,
    );
}

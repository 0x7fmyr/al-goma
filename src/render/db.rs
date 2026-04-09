use ratatui::layout::{Alignment, Constraint, Layout, Margin};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, BorderType::Rounded, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation,
    ScrollbarState,
};
use ratatui::{Frame, layout::Rect};

use super::pop;
use crate::app::{self, AppState};
use crate::ui;

pub fn edit_widow(window: &mut Frame, rect: Rect, app: &mut app::App) {
    let vertical_scroll = app.edit_cursor.scroll;

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

    app.edit_cursor.visable_lines = edit_items[1].height as usize;

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

    for (n, i) in app.db.dishes[app.db_cursor.cursor]
        .ingredients
        .iter()
        .enumerate()
    {
        let c = n;
        let n = n + 1;
        let mut t = Modifier::SLOW_BLINK;

        if app.state == AppState::EditingIngredient
            || app.state == AppState::EditingDishName
            || app.state == AppState::EditingAddIngredient
            || app.state == AppState::PickingCategory
        {
            t = Modifier::empty();
        }

        if c == app.edit_cursor.cursor {
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

        items.push(Span::styled(
            format!(" [{}]", ui::get_category_name(i.category.clone())),
            Style::new().fg(Color::DarkGray),
        ));

        ingredients.push(Line::from(items.clone()));
        items.clear();
    }

    window.render_widget(
        Paragraph::new(header).alignment(Alignment::Center),
        edit_items[0],
    );
    window.render_widget(
        Paragraph::new(ingredients).scroll((vertical_scroll as u16, 0)),
        edit_items[1],
    );

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("•"))
        .end_symbol(Some("•"));

    let mut scrollbar_state =
        ScrollbarState::new(app.db.dishes[app.db_cursor.cursor].ingredients.len())
            .position(vertical_scroll);
    if app.db.dishes[app.db_cursor.cursor].ingredients.len() > app.edit_cursor.visable_lines {
        window.render_stateful_widget(
            scrollbar,
            rect.inner(Margin {
                // using an inner vertical margin of 1 unit makes the scrollbar inside the block
                vertical: 3,
                horizontal: 2,
            }),
            &mut scrollbar_state,
        );
    }
    window.render_widget(
        Paragraph::default().alignment(Alignment::Left).block(
            Block::bordered()
                .border_style(Style::default().fg(Color::Blue))
                .border_type(Rounded)
                .borders(Borders::ALL)
                .title(app.db.dishes[app.db_cursor.cursor].name.to_string())
                .title_alignment(Alignment::Center),
        ),
        rect,
    );
}

pub fn add_dish(window: &mut Frame, rect: Rect, app: &mut app::App, prev_state: AppState) {
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
        );
    }
}

pub fn dish_database(window: &mut Frame, rect: Rect, app: &mut app::App) {
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

            if c == app.db_cursor.cursor {
                let mut t = Modifier::SLOW_BLINK;
                if app.state == AppState::EditingDish
                    || app.state == AppState::EditingIngredient
                    || app.state == AppState::EditingDishName
                    || app.state == AppState::AreYouSureDelDish
                    || app.state == AppState::EditingAddIngredient
                    || app.state == AppState::PickingCategory
                {
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

use ratatui::layout::{Alignment, Constraint, Layout, Margin};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType::Rounded, Borders, Paragraph};
use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState};
use ratatui::{Frame, layout::Rect};

use crate::app;
use crate::items::Category;
use crate::locale::UiText;
use crate::{AppState, };

pub fn new_list(window: &mut Frame, rect: Rect, app: &mut app::App) {
    let chose_window = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Fill(1)])
        .split(rect.inner(Margin {
            horizontal: 6,
            vertical: 2,
        }));

    let buttons = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1)])
        .split(chose_window[1]);

    let mut msg = if app.db.dishes.is_empty() {
        Span::styled(
            app.text_get(UiText::NoDishesInDishtabase),
            Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
        )
    } else {
        Span::raw(app.text_get(UiText::HowManyDishes))
    };

    for c in app.input.chars() {
        if !c.is_numeric() {
            msg = Span::styled(
                app.text_get(UiText::OnlyEnterNums),
                Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
            );
        }
    }

    window.render_widget(
        Paragraph::new(msg).alignment(Alignment::Center),
        chose_window[0],
    );

    let input = vec![
        Span::raw(app.input.to_string()),
        Span::styled(
            "_",
            Style::new()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::SLOW_BLINK),
        ),
    ];

    window.render_widget(
        Paragraph::new(Line::from(input))
            .alignment(Alignment::Center)
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(Color::DarkGray))
                    .border_type(Rounded)
                    .borders(Borders::ALL)
                    .title_alignment(Alignment::Center),
            ),
        buttons[0],
    );

    window.render_widget(
        Paragraph::default().alignment(Alignment::Left).block(
            Block::bordered()
                .border_style(Style::default().fg(Color::LightBlue))
                .border_type(Rounded)
                .borders(Borders::ALL)
                .title_alignment(Alignment::Center),
        ),
        rect,
    );
}

pub fn show_generated_list(window: &mut Frame, rect: Rect, app: &mut app::App) {
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

    window.render_widget(
        Paragraph::default().alignment(Alignment::Left).block(
            Block::bordered()
                .border_style(Style::default().fg(Color::DarkGray).dim())
                .border_type(Rounded)
                .borders(Borders::ALL),
        ),
        edit_window[0],
    );
    let header = Line::from(app.text_get(UiText::Menu))
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::UNDERLINED);

    let mut items: Vec<Span> = Vec::new();
    let mut ingredients: Vec<Line> = Vec::new();

    if let Some(list) = app.current_dish_list.as_mut() {
        for (n, i) in list.iter().enumerate() {
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

            ingredients.push(Line::from(items.clone()));
            items.clear();
        }
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
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    let mut scrollbar_state = ScrollbarState::new(
        app.db.dishes[app.db_cursor.cursor]
            .ingredients
            .len()
            .saturating_sub(app.db_cursor.cursor),
    )
    .position(vertical_scroll);

    window.render_stateful_widget(
        scrollbar,
        rect.inner(Margin {
            // using an inner vertical margin of 1 unit makes the scrollbar inside the block
            vertical: 0,
            horizontal: 0,
        }),
        &mut scrollbar_state,
    );

    window.render_widget(
        Paragraph::default().alignment(Alignment::Left).block(
            Block::bordered()
                .border_style(Style::default().fg(Color::Blue))
                .border_type(Rounded)
                .borders(Borders::ALL)
                .title(app.text_get(UiText::GeneratedList))
                .title_alignment(Alignment::Center),
        ),
        rect,
    );
}

pub fn show_generated_list_ingredients(window: &mut Frame, rect: Rect, app: &mut app::App) {
    let list_window = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Fill(1)])
        .split(rect.inner(Margin {
            horizontal: 3,
            vertical: 1,
        }));

    app.db_cursor.visable_lines = list_window[1].height as usize;

    window.render_widget(
        Paragraph::new(
            Line::from(app.text_get(UiText::ShoppingList))
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::UNDERLINED),
        )
        .alignment(Alignment::Center),
        list_window[0],
    );

    let mut spans: Vec<Span> = Vec::new();
    let mut ingredients: Vec<Line> = Vec::new();
    let mut prev_category = Category::Vegetables;
    let mut veg_been_done = false;
    let mut cs = 0;
    let mut selected_cat: Category;

    for (i, ing) in app.shopping_list.iter().enumerate() {
        let mut space = "                         ".to_string();
        let c = i;
        let i = i + 1;
        selected_cat = ing.category;

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

            space.drain(..3);

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
                ing.name.clone(),
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

            spans.push(Span::raw(ing.name.clone()));
        }

        for _ in ing.name.chars() {
            space.pop();
        }

        if i > 9 {
            space.pop();
        }

        if ing.category != prev_category
            || (prev_category == Category::Vegetables && !veg_been_done)
        {
            spans.push(Span::raw(space));
            spans.push(
                Span::styled(
                    app.get_category_name(ing.category),
                    Style::new().fg(Color::DarkGray),
                )
                .add_modifier(Modifier::BOLD),
            );
        } else if ing.category == prev_category {
            let cat_count = app.get_category_name(prev_category).len();

            while cs < cat_count {
                space.push(' ');
                cs += 1
            }

            if selected_cat == ing.category {
                space.pop();
            }

            let mut margin = "│";

            if i < app.shopping_list.len() {
                let next_cat = app.shopping_list[i].category;
                if next_cat != ing.category {
                    space.drain(..2);
                    margin = "──┘";
                }
            } else if i == app.shopping_list.len() {
                space.drain(..2);
                margin = "──┘";
            }

            spans.push(Span::raw(space));
            spans.push(Span::styled(margin, Style::new().fg(Color::DarkGray)));
        }
        cs = 0;
        prev_category = ing.category;
        veg_been_done = true;

        ingredients.push(Line::from(spans.clone()));
        spans.clear();
    }

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("•"))
        .end_symbol(Some("•"));

    let vertical_scroll = app.db_cursor.scroll;

    let mut scrollbar_state =
        ScrollbarState::new(app.shopping_list.len().saturating_sub(app.db_cursor.scroll))
            .position(vertical_scroll);

    window.render_widget(
        Paragraph::new(ingredients)
            .scroll((vertical_scroll as u16, 0))
            .alignment(Alignment::Left),
        list_window[1],
    );
    if app.shopping_list.len() > app.db_cursor.visable_lines {
        window.render_stateful_widget(
            scrollbar,
            rect.inner(Margin {
                // using an inner vertical margin of 1 unit makes the scrollbar inside the block
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
    }
}

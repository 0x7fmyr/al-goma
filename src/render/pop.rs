use ratatui::layout::{Alignment, Constraint, Layout, Margin};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType::Rounded, Borders, Paragraph};
use ratatui::{Frame, layout::Rect};

use crate::app::{self, AppState};

pub fn are_you_sure(window: &mut Frame, rect: Rect, app: &mut app::App, msg: Vec<Line>) {
    let del_window = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(4),
            Constraint::Fill(1),
        ])
        .split(rect.inner(Margin {
            horizontal: 5,
            vertical: 1,
        }));

    let buttons = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(6),
            Constraint::Fill(1),
        ])
        .split(del_window[2]);

    window.render_widget(
        Paragraph::new(msg).alignment(Alignment::Center),
        del_window[1],
    );

    let mut yes = Line::from("Yes");
    let mut no = Line::from("No");

    if app.ays_cursor == 0 {
        yes = Line::from(Span::styled(
            "Yes",
            Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
        ));
    } else {
        no = Line::from(Span::styled(
            "No",
            Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
        ));
    }

    window.render_widget(
        Paragraph::new(yes).alignment(Alignment::Center).block(
            Block::bordered()
                .border_style(Style::default().fg(Color::Red))
                .border_type(Rounded)
                .borders(Borders::ALL)
                .title_alignment(Alignment::Center),
        ),
        buttons[0],
    );

    window.render_widget(
        Paragraph::new(no).alignment(Alignment::Center).block(
            Block::bordered()
                .border_style(Style::default().fg(Color::LightRed))
                .border_type(Rounded)
                .borders(Borders::ALL)
                .title_alignment(Alignment::Center),
        ),
        buttons[2],
    );

    window.render_widget(
        Paragraph::default().alignment(Alignment::Left).block(
            Block::bordered()
                .border_style(Style::default().fg(Color::Red))
                .border_type(Rounded)
                .borders(Borders::ALL)
                .title_alignment(Alignment::Center),
        ),
        rect,
    );
}

pub fn pick_category(
    window: &mut Frame,
    rect: Rect,
    app: &mut app::App,
    s: AppState,
    name: String,
) {
    let pick_window = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Fill(1)])
        .split(rect.inner(Margin {
            horizontal: 5,
            vertical: 1,
        }));

    if s == AppState::EnteringIngredients || s == AppState::AddToShoppingList {
        let msg = vec![
            Line::from("I can't find a category for:"),
            Line::from(app::uppercase_words(&name)).add_modifier(Modifier::BOLD),
            Line::from("Please choose one:"),
        ];
        window.render_widget(
            Paragraph::new(msg).alignment(Alignment::Center),
            pick_window[0],
        );
    }
    if s == AppState::EditingDish {
        let msg = vec![
            Line::from("Please choose category for:"),
            Line::from(
                app.db.dishes[app.db_cursor.cursor].ingredients[app.edit_cursor.cursor]
                    .name
                    .to_string(),
            )
            .add_modifier(Modifier::BOLD),
        ];
        window.render_widget(
            Paragraph::new(msg).alignment(Alignment::Center),
            pick_window[0],
        );
    }

    let categories = [
        "Annat",
        "Grönsaker",
        "Frukt",
        "Mejeri",
        "Protein",
        "Skafferi/Torr varor",
        "Kryddor",
    ];

    let mut line: Vec<Line> = Vec::new();
    for (i, c) in categories.iter().enumerate() {
        if i == app.picking_cursor {
            line.push(Line::from(Span::styled(
                c.to_string(),
                Style::new()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            )));
        } else {
            line.push(Line::from(Span::styled(
                c.to_string(),
                Style::new().fg(Color::DarkGray),
            )));
        }
    }
    window.render_widget(
        Paragraph::new(line).alignment(Alignment::Center),
        pick_window[1],
    );

    window.render_widget(
        Paragraph::default().alignment(Alignment::Left).block(
            Block::bordered()
                .border_style(Style::default().fg(Color::LightCyan))
                .border_type(Rounded)
                .borders(Borders::ALL)
                .title_alignment(Alignment::Center),
        ),
        rect,
    );
}

pub fn input_box(window: &mut Frame, rect: Rect, app: &mut app::App, title_msg: String) {
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
                    .border_style(Style::default().fg(Color::LightBlue))
                    .border_type(Rounded)
                    .borders(Borders::ALL)
                    .title(title_msg)
                    .title_alignment(Alignment::Center),
            ),
        rect,
    );
}

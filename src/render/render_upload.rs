use ratatui::layout::{Alignment, Constraint, Layout, Margin};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};

use ratatui::widgets::{Block, BorderType::Rounded, Borders, Clear, Paragraph, Wrap};
use ratatui::{Frame, layout::Rect};

use crate::app::{self, AppState};
use crate::locale::UiText;
use crate::render::pop;

pub fn login_popup(window: &mut Frame, rect: Rect, msg: Vec<Line>) {
    let main_h = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(40),
            Constraint::Fill(1),
        ])
        .split(rect);

    let main_v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(10),
            Constraint::Fill(1),
        ])
        .split(main_h[1]);

    window.render_widget(Clear, main_v[1]);

    window.render_widget(
        Paragraph::default().alignment(Alignment::Center).block(
            Block::bordered()
                .border_style(Style::default().fg(Color::Cyan))
                .border_type(Rounded)
                .borders(Borders::ALL),
        ),
        main_v[1],
    );

    window.render_widget(
        Paragraph::new(msg).alignment(Alignment::Center),
        main_v[1].inner(Margin {
            horizontal: 1,
            vertical: 2,
        }),
    );
}

pub fn show_login_url(window: &mut Frame, rect: Rect, app: &mut app::App) {
    let msg = vec![
        Line::from(app.text_get(UiText::UPGo2ThisUrl)),
        Line::from(app.text_get(UiText::UPGo2ThisUrlToolTip)),
    ];
    let url = app.login_url.clone().unwrap();

    let text = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(rect);

    window.render_widget(
        Paragraph::new(msg[0].clone()).alignment(Alignment::Center),
        text[0],
    );

    window.render_widget(
        Paragraph::new(url)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false }),
        text[1],
    );

    window.render_widget(
        Paragraph::new(msg[1].clone()).alignment(Alignment::Center),
        text[4],
    );

    if matches!(app.state, AppState::UploadEnterCode) {
        pop::input_box(window, text[2], app, app.text_get(UiText::UPEnterCode));
    }
}

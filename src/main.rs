use crossterm::ExecutableCommand;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::Terminal;
use ratatui::layout::{Constraint, Direction, Layout, Margin};
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use ratatui::prelude::CrosstermBackend;

use std::error::Error;
use std::io::stdout;

use crate::app::AppState;

mod app;
mod db;
mod items;
mod list;
mod locale;
mod render;
mod ui;

//claude --resume ce8bf707-c036-400b-a940-359d721e90bc
fn main() -> Result<(), Box<dyn Error>> {
    //set up
    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = app::App::init();
    let result = run(&mut terminal, &mut app);

    //clean up
    disable_raw_mode()?;
    std::io::stdout().execute(LeaveAlternateScreen)?;
    result
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut app::App,
) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|window| {
            if window.area().height <= 20 || window.area().width <= 75 {
                let edge_h = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Fill(1),
                        Constraint::Length(39), // main window
                        Constraint::Fill(1),
                    ])
                    .split(window.area());

                let edge_v = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Fill(1),
                        Constraint::Length(1),
                        Constraint::Fill(1),
                    ])
                    .split(edge_h[1]);

                window.render_widget(
                    Paragraph::new("You're gonna need a bigger terminal..."),
                    edge_v[1],
                );
            } else {
                let edge_w = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(1),
                        Constraint::Fill(1),
                        Constraint::Length(1),
                    ])
                    .split(window.area());

                let edge_w = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length(2),
                        Constraint::Fill(1), // main window
                        Constraint::Length(2),
                    ])
                    .split(edge_w[1]);

                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length(30),
                        Constraint::Length(2),
                        Constraint::Fill(1),
                    ])
                    .split(edge_w[1].inner(Margin {
                        horizontal: 3,
                        vertical: 2,
                    }));

                // MAIN WINDOW
                window.render_widget(
                    Paragraph::default().block(
                        Block::bordered()
                            .border_style(Style::default().fg(Color::Blue))
                            .border_type(BorderType::Thick)
                            .borders(Borders::ALL),
                    ),
                    edge_w[1],
                );

                render::main_window::left(window, chunks[0], app);

                render::main_window::right(window, chunks[2], app);
            }
        })?;
        if event::poll(std::time::Duration::from_millis(16))?
            && let Event::Key(key) = event::read()?
        {
            match key {
                //mod presses
                KeyEvent {
                    //todo! fix this is a global command no good

                    code: KeyCode::Char('s'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    if app.pending_dish.is_some() {
                        app.push_dish_to_db()
                    }
                }
                KeyEvent {
                    //todo! fix this is a global command no good
                    code: KeyCode::Char('n'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    app.state = app::AppState::EditingDishName;
                    app.pending_dish = Some(app.db.dishes[app.db_cursor.cursor].to_owned());
                }
                KeyEvent {
                    code: KeyCode::Char('a'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    if app.state == AppState::EditingDish {
                        app.state = AppState::EditingAddIngredient;
                    } else if app.state == AppState::ShowShoppingList {
                        app.state = AppState::AddToShoppingList
                    }
                }
                KeyEvent {
                    code: KeyCode::Char('k'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    if app.state == AppState::EditingDish {
                        app.pending_dish = Some(app.db.dishes[app.db_cursor.cursor].clone());
                        app.prev_state = Some(app.state);
                        app.state = AppState::PickingCategory;
                    }
                }
                KeyEvent {
                    code: KeyCode::Char('p'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => app.state = AppState::PromptPrint,
                //plain keypresses
                KeyEvent { code, .. } => match code {
                    KeyCode::Char('q') => match app.state {
                        AppState::EnteringDishName
                        | AppState::EnteringIngredients
                        | AppState::EditingIngredient
                        | AppState::EditingDishName
                        | AppState::EditingAddIngredient
                        | AppState::NewList
                        | AppState::AddToShoppingList => app.keyboard_input('q'),
                        _ => break,
                    },
                    KeyCode::Esc => app.handle_esc(),

                    KeyCode::Down => app.move_cursor_down(),
                    KeyCode::Up => app.move_cursor_up(),
                    KeyCode::Left => app.move_focus_left(),
                    KeyCode::Right => app.move_focus_right(),
                    KeyCode::Char('p') => {
                        if matches!(app.state, AppState::PromptPrint) {
                            app::print_shopping_list_txt_file(
                                app.shopping_list.clone(),
                                app.text_options.0,
                                app.text_options.1,
                            )
                            .expect("failed to print file..");
                            app.state = AppState::ShowShoppingList
                        } else {
                            app.keyboard_input('p')
                        }
                    }

                    KeyCode::Char(c) => app.keyboard_input(c),

                    KeyCode::Backspace => app.backspace(),
                    KeyCode::Delete => app.handle_delete(),

                    KeyCode::Enter => app.handle_enter(),

                    _ => {}
                },
            }
        }
    }
    Ok(())
}

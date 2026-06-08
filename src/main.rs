use chrono::Utc;
use clap::Parser;
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    layout::{Constraint, Direction, Layout, Margin},
    prelude::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::{error::Error, io::stdout};

use crate::{app::AppState, lists::upload::UploadProgress};
mod app;
mod dishtabase;
mod lists;

mod interface;
mod render;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    setup: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    if args.setup {
        //setup::run_setup();
        return Ok(());
    }

    // Tui set up
    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = app::App::init();
    let result = run(&mut terminal, &mut app);

    //Tui clean up
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
            if matches!(app.state, AppState::UploadShowLoginUrl)
                || matches!(app.state, AppState::UploadEnterCode)
            {
                //UploadShowLoginUrl kill main ui in favor of only text
                render::render_upload::show_login_url(window, window.area(), app);
            } else {
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
            }
        })?;

        if matches!(app.state, AppState::UploadWaitingForLoginUrl) {
            // Polling url_receiver if google has sent the link

            if let Some(receiver) = &mut app.url_receiver
                && let Ok(url) = receiver.try_recv()
            {
                app.login_url = Some(url);
                app.state = AppState::UploadShowLoginUrl
            }
        }

        if matches!(app.state, AppState::UploadLogginginWait) {
            // Polling to see if loggin is successful
            if let Some(receiver) = &mut app.login_result_receiver {
                match receiver.try_recv() {
                    Ok(Ok(())) => {
                        app.input = format!("Shopping List {}", Utc::now().date_naive());
                        app.state = AppState::UploadMenu;
                    }
                    Ok(Err(e)) => {
                        app.err_msg = Some(e.to_string());
                        app.state = AppState::Error;
                    }
                    Err(_) => {}
                }
            }
        }

        if matches!(app.state, AppState::Uploading) {
            // Polling for Error handling in upload()
            if let Some(upload_result_reciver) = &mut app.upload_result_receiver {
                match upload_result_reciver.try_recv() {
                    Ok(Ok(())) => {}
                    Ok(Err(e)) => {
                        app.err_msg = Some(e.to_string());
                        app.state = AppState::Error;
                    }
                    Err(_) => {}
                }
            }
            // Polling progress_checker_receinver to know what to update the progressbar in upload
            if let Some(progess_receiver) = &mut app.progress_checker_receiver
                && let Ok(i) = progess_receiver.try_recv()
            {
                app.progress = i
            }

            if app.progress.done {
                // Upload is done
                app.state = AppState::UploadDone;
                app.progress = UploadProgress {
                    procent: 0.0,
                    done: false,
                }
            }
        }

        if event::poll(std::time::Duration::from_millis(16))?
            && let Event::Key(key) = event::read()?
        {
            match key {
                // Mod Presses
                KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    if app.pending_dish.is_some()
                        && (matches!(app.state, AppState::EnteringIngredients)
                            || matches!(app.state, AppState::EditingDishName))
                    {
                        app.push_dish_to_db()
                    }
                }
                KeyEvent {
                    code: KeyCode::Char('n'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    if app.state == AppState::ViewingDatabase {
                        app.state = app::AppState::EditingDishName;
                        app.pending_dish = Some(app.db.dishes[app.db_cursor.cursor].to_owned());
                        app.input = app.pending_dish.as_ref().unwrap().name.clone();
                    }
                }
                KeyEvent {
                    code: KeyCode::Char('a'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    if app.state == AppState::EditingDish {
                        app.state = AppState::EditingAddIngredient;
                    } else if app.state == AppState::ShowGeneratedList {
                        app.prev_state = Some(app.state);
                        app.state = AppState::AddToGeneratedList
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
                } => {
                    if matches!(app.state, AppState::ShowShoppingList) {
                        app.state = AppState::PromptPrint
                    }
                }
                KeyEvent {
                    code: KeyCode::Backspace,
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    app.backspace(true);
                }
                KeyEvent {
                    code: KeyCode::Char('h'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    app.backspace(true);
                }

                // Plain Keypresses
                KeyEvent { code, .. } => match code {
                    KeyCode::Char('q') => match app.state {
                        AppState::EnteringDishName
                        | AppState::EnteringIngredients
                        | AppState::EditingIngredient
                        | AppState::EditingDishName
                        | AppState::EditingAddIngredient
                        | AppState::NewList
                        | AppState::AddToShoppingList
                        | AppState::UploadEnterCode => app.char_input('q'),
                        _ => break,
                    },
                    KeyCode::Esc => app.handle_esc(),

                    KeyCode::Down => app.move_cursor_down(),
                    KeyCode::Up => app.move_cursor_up(),
                    KeyCode::Left => app.move_focus_left(),
                    KeyCode::Right => app.move_focus_right(),
                    KeyCode::Char('p') => {
                        if app.state == AppState::PromptPrint {
                            app.print_shopping_list_txt_file(
                                app.shopping_list.clone(),
                                app.text_options.0,
                                app.text_options.1,
                            )
                            .expect("failed to print file..");
                            app.state = AppState::ShowShoppingList
                        } else if matches!(app.state, AppState::UploadShowLoginUrl) {
                            app.state = AppState::UploadEnterCode;
                            match app::paste_from_clipboard() {
                                Ok(paste) => app.input = paste,
                                Err(e) => app.err_msg = Some(e),
                            }
                        } else {
                            app.char_input('p')
                        }
                    }
                    KeyCode::Char('c') => {
                        if app.state == AppState::UploadShowLoginUrl {
                            app::copy_to_clipboard(app.login_url.clone().unwrap()).ok();
                        } else {
                            app.char_input('c');
                        }
                    }

                    KeyCode::Char(ch) => app.char_input(ch),

                    KeyCode::Backspace => app.backspace(false),
                    KeyCode::Delete => app.handle_delete(),
                    KeyCode::Enter => app.handle_enter(),
                    KeyCode::Tab => app.handle_tab(),

                    _ => {}
                },
            }

            //
        }
    }
    Ok(())
}

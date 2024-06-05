mod app;
mod leitner;
mod ui;

use crate::{
    app::{App, CurrentScreen},
    ui::ui,
};
use clap::{Parser, Subcommand};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::path::Path;
use std::{error::Error, io};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    deck_path: String,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Learn,
    Import { csv_path: String },
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        let _ = terminal.draw(|f| ui(f, app));

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_queue {
                Some(current_queue) => match app.current_screen {
                    CurrentScreen::Asking => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        _ => {
                            app.current_screen = CurrentScreen::Checking;
                        }
                    },
                    CurrentScreen::Checking => {
                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char('y') => app.process(true),
                            KeyCode::Char('n') => app.process(false),
                            _ => {}
                        };
                        app.current_screen = CurrentScreen::Asking;
                    }
                },
                None => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('r') => app.refill(),
                    _ => {}
                },
            }
        }
    }
}

fn learn(app: &mut App) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    if let Err(err) = run_app(&mut terminal, app) {
        eprintln!("{err:?}")
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Args::parse();
    let path = Path::new(&cli.deck_path);
    let mut app = App::new(path)?;

    match cli.command {
        Command::Learn => {
            if let Err(err) = learn(&mut app) {
                println!("{err:?}");
            }
        }
        Command::Import { csv_path } => {
            if let Ok(imported) = app.deck.import(Path::new(&csv_path)) {
                println!("Imported {} cards.", imported)
            }
        }
    };

    Ok(app.deck.save(path)?)
}

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

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    deck_path: String,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
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
            if key.code == KeyCode::Char('q') {
                return Ok(());
            }
            match (&app.current_screen, app.current_queue) {
                (CurrentScreen::Asking, Some(_)) => app.current_screen = CurrentScreen::Checking,
                (CurrentScreen::Checking, Some(_)) => {
                    match key.code {
                        KeyCode::Char('y') => app.process(true),
                        KeyCode::Char('n') => app.process(false),
                        _ => {}
                    };
                    app.current_screen = CurrentScreen::Asking;
                }
                (_, None) => match key.code {
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
        None => learn(&mut app)?,
        Some(Command::Import { csv_path }) => {
            let imported = app.deck.import(Path::new(&csv_path))?;
            println!("Imported {} cards.", imported)
        }
    };

    Ok(app.deck.save(path)?)
}

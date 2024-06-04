mod app;
mod ui;
mod leitner;

use crate::{app::{App, CurrentScreen}, ui::ui};
use std::path::Path;
use clap::Parser;
use std::{error::Error, io};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    deck_path: String,
}


fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
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
            KeyCode::Char('q') => return Ok(true),
            _ => {
              app.current_screen = CurrentScreen::Checking;
            }
          },
          CurrentScreen::Checking => {
            match key.code {
              KeyCode::Char('q') => return Ok(true),
              KeyCode::Char('y') => {
                app.process(true)
              },
              KeyCode::Char('n') => {
                app.process(false)
              },
              _ => {}
            };
            app.current_screen = CurrentScreen::Asking;
          }
        },
        None => return Ok(true)
      }
    }
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let cli = Args::parse();
  let path = Path::new(&cli.deck_path);

  enable_raw_mode()?;
  let mut stderr = io::stderr();
  execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

  let backend = CrosstermBackend::new(stderr);
  let mut terminal = Terminal::new(backend)?;
  let mut app = App::new(path)?;

  let res = run_app(&mut terminal, &mut app);

  disable_raw_mode()?;
  execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
  )?;
  terminal.show_cursor()?;

  if let Ok(_) = res {
    let _ = app.deck.save(path);
  } else if let Err(err) = res {
      println!("{err:?}");
  }
  Ok(())
}

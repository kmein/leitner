use crate::leitner::Deck;
use std::io;

pub enum CurrentScreen {
  Asking,
  Checking,
}

pub struct App {
  pub deck: Deck,
  pub current_queue: Option<usize>,
  pub current_screen: CurrentScreen,
}

impl App {
  pub fn new() -> io::Result<App> {
    let deck = Deck::load()?;
    let current_queue = deck.get_next_queue();
    Ok(App {
      deck,
      current_queue,
      current_screen: CurrentScreen::Asking,
    })
  }
}

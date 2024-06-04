use crate::leitner::Deck;
use std::path::Path;
use std::io;

pub enum CurrentScreen {
  Asking,
  Checking,
}

pub struct App {
  pub file_name: String,
  pub deck: Deck,
  pub current_queue: Option<usize>,
  pub current_screen: CurrentScreen,
}

impl App {
  pub fn new(path: &Path) -> io::Result<App> {
    let deck = Deck::load(path)?;
    let current_queue = deck.get_next_queue();
    Ok(App {
      file_name: path.to_str().expect("Hä?").to_string(),
      deck,
      current_queue,
      current_screen: CurrentScreen::Asking,
    })
  }

  pub fn process(&mut self, did_know: bool) {
    if let Some(queue) = self.current_queue {
      self.deck.process(queue, did_know);
      self.current_queue = self.deck.get_next_queue();
    }
  }
}

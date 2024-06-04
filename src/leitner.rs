use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;
use std::collections::VecDeque;
use uuid::Uuid;

const CARD_THICKNESS_MM: f32 = 0.5;

const CARDS_PER_CM: usize = (10 as f32 / CARD_THICKNESS_MM) as usize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Card {
  id: String,
  pub front: String,
  pub back: String,
}

impl Card {
  pub fn new(front: String, back: String) -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
      front,
      back,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Queue {
  pub cards: VecDeque<Card>,
  capacity: usize,
}

impl Queue {
  fn new(size_cm: usize) -> Self {
    let capacity = CARDS_PER_CM * size_cm;
    Self {
      cards: VecDeque::with_capacity(capacity),
      capacity
    }
  }

  fn free_space(&self) -> usize {
    self.capacity - self.cards.len()
  }

  pub fn get_next_card(&self) -> Option<&Card> {
    self.cards.get(0)
  }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deck {
  pub queues: Vec<Queue>
}

impl Deck {
  fn new() -> Self {
    Self {
      queues: vec![
        Queue::new(1),
        Queue::new(2),
        Queue::new(5),
        Queue::new(8),
        Queue::new(14),
      ]
    }
  }

  pub fn load(path: &Path) -> io::Result<Self> {
    if path.exists() {
      let file = File::open(path)?;
      let reader = BufReader::new(file);
      Ok(serde_json::from_reader(reader)?)
    } else {
      Ok(Self::new()) // Default to 5 queues
    }
  }

  pub fn save(&self, path: &Path) -> io::Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, self)?;
    Ok(())
  }

  pub fn add_card(&mut self, card: Card) -> Option<()> {
    let q1 = &mut self.queues[0];
    if q1.free_space() > 0 {
      q1.cards.push_back(card);
      Some(())
    } else {
      None
    }
  }

  pub fn process(&mut self, queue: usize, did_know: bool) {
    let card = self.queues[queue].cards.pop_front().expect("Hä?");
    if !did_know {
      self.queues[0].cards.push_back(card)
    } else if queue < self.queues.len()  - 1 {
      self.queues[queue+1].cards.push_back(card)
    }
    // else (if alread on queue 5): do not add the card back i.e. delete it
  }

  pub fn get_next_queue(&self) -> Option<usize> {
    if self.queues[0].cards.len() > 3 { Some(0) }
    else {
      self.queues.iter().position(|q| q.free_space() < CARDS_PER_CM)
    }
  }
}


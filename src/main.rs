use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write, Read, Stdin, Stdout};
use std::path::Path;
use std::collections::VecDeque;
use uuid::Uuid;

const DATA_FILE: &str = "flashcards.json";

const CARD_THICKNESS_MM: f32 = 0.5;

const CARDS_PER_CM: usize = (10 as f32 / CARD_THICKNESS_MM) as usize;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Card {
  id: String,
  front: String,
  back: String,
}

impl Card {
  fn new(front: String, back: String) -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
      front,
      back,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct Queue {
  cards: VecDeque<Card>,
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

  fn get_next_card(&self) -> Option<&Card> {
    self.cards.get(0)
  }

}

#[derive(Debug, Serialize, Deserialize)]
struct Deck {
  queues: Vec<Queue>
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

  fn load() -> io::Result<Self> {
    if Path::new(DATA_FILE).exists() {
      let file = File::open(DATA_FILE)?;
      let reader = BufReader::new(file);
      Ok(serde_json::from_reader(reader)?)
    } else {
      Ok(Self::new()) // Default to 5 queues
    }
  }

  fn save(&self) -> io::Result<()> {
    let file = File::create(DATA_FILE)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, self)?;
    Ok(())
  }

  fn add_card(&mut self, card: Card) -> Option<()> {
    let q1 = &mut self.queues[0];
    if q1.free_space() > 0 {
      q1.cards.push_back(card);
      Some(())
    } else {
      None
    }
  }

  fn process(&mut self, queue: usize, did_know: bool) {
    let card = self.queues[queue].cards.pop_front().expect("Hä?");
    self.queues[
      if did_know { queue+1 } else { 0 }
    ].cards.push_back(card);
  }

  fn get_next_queue(&self) -> Option<usize> {
    if self.queues[0].cards.len() > 3 { Some(0) }
    else {
      self.queues.iter().position(|q| q.free_space() < CARDS_PER_CM)
    }
  }
}

fn wait_key(stdout: &mut Stdout, stdin: &mut Stdin) {
  write!(stdout, "Press any key to continue...").unwrap();
  stdout.flush().unwrap();
  stdin.read(&mut [0u8]).unwrap();
}

fn did_know(stdout: &mut Stdout, stdin: &mut Stdin) -> bool {
  write!(stdout, "Did you know this? y/n").unwrap();
  stdout.flush().unwrap();
  let key = stdin.read(&mut [0u8]).unwrap();
  key as u32 != u32::from('n')
}

fn main() -> io::Result<()> {
  let mut stdin = io::stdin();
  let mut stdout = io::stdout();
  let mut deck = Deck::load()?;

  for x in 1..10 {
    for y in 1..10 {
      deck.add_card(Card::new(
        format!("{} × {} = ?", x, y),
        format!("{}", x * y),
      ));
    }
  }

  deck.save()?;

  while let Some(queue) = deck.get_next_queue() {
    if let Some(card) = deck.queues[queue].get_next_card() {

      println!("{}\n", card.front);
      wait_key(&mut stdout, &mut stdin);
      println!("{}\n", card.back);

      let known = did_know(&mut stdout, &mut stdin);
      deck.process(queue, known);
    }
  }

  deck.save()?;
  Ok(())
}

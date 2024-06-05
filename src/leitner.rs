use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

const CARD_THICKNESS_MM: f32 = 0.5;

const CARDS_PER_CM: usize = (10 as f32 / CARD_THICKNESS_MM) as usize;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Card {
    pub front: String,
    pub back: String,
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
            capacity,
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
    stash: Vec<Card>,
    done: Vec<Card>,
    pub queues: Vec<Queue>,
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
            ],
            stash: Vec::new(),
            done: Vec::new(),
        }
    }

    fn card_exists(&self, card: &Card) -> bool {
        self.stash.contains(&card) || self.queues.iter().any(|q| q.cards.contains(&card))
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

    pub fn import(&mut self, csv_path: &Path) -> io::Result<usize> {
        if csv_path.exists() {
            println!("loading from csv");
            let initial_stash_size = self.stash.len();
            let mut rdr = ReaderBuilder::new().delimiter(b',').from_path(csv_path)?;
            for result in rdr.deserialize() {
                if let Ok(card) = result {
                    if !self.card_exists(&card) {
                        self.stash.push(card);
                    } else {
                        eprintln!("Card already exists: {:?}", card)
                    }
                } else {
                    eprintln!("Malformed record: {:?}", result)
                }
            }
            Ok(self.stash.len() - initial_stash_size)
        } else {
            panic!("File {} does not exist.", csv_path.display());
        }
    }

    pub fn can_refill(&self) -> bool {
        self.queues[0].free_space() > 0 && self.stash.len() > 0
    }

    pub fn stash_size(&self) -> usize {
        self.stash.len()
    }

    pub fn refill(&mut self) {
        while self.can_refill() {
            let card = self.stash.pop().expect("The stash should have cards.");
            self.queues[0].cards.push_back(card);
        }
    }

    pub fn save(&self, path: &Path) -> io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    pub fn process(&mut self, queue: usize, did_know: bool) {
        let card = self.queues[queue].cards.pop_front().expect("Hä?");
        if !did_know {
            self.queues[0].cards.push_back(card)
        } else if queue < self.queues.len() - 1 {
            self.queues[queue + 1].cards.push_back(card)
        } else {
            self.done.push(card)
        }
    }

    pub fn get_next_queue(&self) -> Option<usize> {
        if self.queues[0].cards.len() > 3 {
            Some(0)
        } else {
            self.queues
                .iter()
                .position(|q| q.free_space() < CARDS_PER_CM)
        }
    }
}

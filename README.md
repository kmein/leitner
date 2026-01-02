# Leitner

A terminal-based flashcard application implementing the Leitner spaced repetition system, written in Rust.

## What is the Leitner System?

The Leitner system is a proven method for learning and memorizing information using flashcards. It uses spaced repetition to maximize learning efficiency:

- Cards are organized into multiple queues (boxes) with increasing capacity
- When you answer correctly, the card moves to the next queue with longer intervals
- When you answer incorrectly, the card returns to the first queue for more frequent review
- Cards that reach the final queue are considered "learned"

This implementation uses 5 queues with capacities based on physical card thickness (0.5mm), simulating a physical Leitner box:
- Queue 1: 1 cm (20 cards)
- Queue 2: 2 cm (40 cards)
- Queue 3: 5 cm (100 cards)
- Queue 4: 8 cm (160 cards)
- Queue 5: 14 cm (280 cards)

## Features

- 📚 Interactive terminal UI for studying flashcards
- 📥 Import flashcards from CSV files
- 💾 Automatic progress saving in JSON format
- 🔄 Spaced repetition algorithm
- 🎯 Queue-based learning system
- 🐍 Python utility for converting poems to flashcards

## Installation

### Prerequisites

- Rust (edition 2021 or later)
- Cargo

### Building from Source

```bash
git clone https://github.com/kmein/leitner
cd leitner
cargo build --release
```

The binary will be available at `target/release/leitner`.

### Using Nix

If you have Nix with flakes enabled:

```bash
nix develop
```

## Usage

### Starting a Learning Session

```bash
leitner <deck_path>
```

Example:
```bash
leitner flashcards.json
```

If the deck file doesn't exist, a new one will be created.

### Controls

- **Any key**: Reveal the answer (when viewing question)
- **y**: Mark as correct (when viewing answer)
- **n**: Mark as incorrect (when viewing answer)
- **r**: Refill the first queue from the stash
- **q**: Quit and save progress

### Importing Flashcards from CSV

```bash
leitner <deck_path> import <csv_path>
```

Example:
```bash
leitner flashcards.json import my_cards.csv
```

The CSV file should have two columns: `front` and `back`:

```csv
front,back
"2 × 2 = ?","4"
"Capital of France?","Paris"
```

### Converting Poems to Flashcards

Use the included Python utility to convert a poem into sequential flashcards:

```bash
cat poem.txt | python3 tools/poem2csv.py > poem.csv
leitner flashcards.json import poem.csv
```

This creates cards where each paragraph leads to the next, helping you memorize the poem sequentially.

## File Format

The deck is stored as a JSON file with the following structure:

```json
{
  "stash": [],
  "done": [],
  "queues": [
    {
      "cards": [
        {
          "front": "Question text",
          "back": "Answer text"
        }
      ],
      "capacity": 20
    }
  ]
}
```

- **stash**: New cards waiting to be added to queue 1
- **done**: Cards that have been fully learned
- **queues**: The 5 learning queues with their cards and capacities

## Dependencies

- [serde](https://serde.rs/) - Serialization/deserialization
- [serde_json](https://github.com/serde-rs/json) - JSON support
- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation
- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing
- [csv](https://github.com/BurntSushi/rust-csv) - CSV parsing

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

See the repository for license information.

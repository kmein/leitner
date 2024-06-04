use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen};

pub fn ui(f: &mut Frame, app: &App) {
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
       Constraint::Length(3),
       Constraint::Min(1),
       Constraint::Length(3),
    ])
    .split(f.size());

  let title_block = Block::default()
    .borders(Borders::ALL)
    .style(Style::default());

  let title = Paragraph::new(Text::styled(
      "Create New Json",
      Style::default().fg(Color::Green),
      ))
    .block(title_block);

  f.render_widget(title, chunks[0]);

  let widget = match app.current_queue {
    None => {
      Span::styled("Nothing to learn", Style::default().fg(Color::Yellow))
    },
    Some(queue) => {
      if let Some(card) = app.deck.queues[queue].get_next_card() {
        match app.current_screen {
          CurrentScreen::Asking => Span::styled(card.front.to_string(), Style::default()),
          CurrentScreen::Checking => Span::styled(card.back.to_string(), Style::default()),
        }
      } else {
        Span::styled("Something went wrong :(", Style::default().fg(Color::Red))
      }
    }
  };

  f.render_widget(Paragraph::new(widget).wrap(Wrap{trim: false}), chunks[1]);

  let current_keys_hint = match app.current_queue {
    None => Span::styled("(q) quit", Style::default()),
    Some(queue) => match app.current_screen {
      CurrentScreen::Asking => Span::styled("Do you know this?", Style::default()),
      CurrentScreen::Checking => Span::styled("Did you know this? (y) yes (n) no", Style::default()),
    }
  };

  let queues_footer = Paragraph::new(Line::from(
      Span::styled(format!("{} {} {} {} {}",
                           app.deck.queues[0].cards.len(),
                           app.deck.queues[1].cards.len(),
                           app.deck.queues[2].cards.len(),
                           app.deck.queues[3].cards.len(),
                           app.deck.queues[4].cards.len(),
                           ), Style::default())
  )).block(Block::default().borders(Borders::ALL));

  let key_notes_footer =
    Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

  let footer_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
    .split(chunks[2]);

  f.render_widget(queues_footer, footer_chunks[0]);
  f.render_widget(key_notes_footer, footer_chunks[1]);
}

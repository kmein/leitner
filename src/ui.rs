use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap, Padding},
    Frame,
};

use crate::app::{App, CurrentScreen};

pub fn ui(f: &mut Frame, app: &App) {
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
       Constraint::Length(3),
       Constraint::Min(3),
       Constraint::Min(3),
       Constraint::Length(3),
    ])
    .split(f.size());

  let title_block = Block::default()
    .borders(Borders::ALL)
    .style(Style::default());

  let title = Paragraph::new(Text::styled(
      &app.file_name,
      Style::default().fg(Color::Green),
      ))
    .block(title_block);

  f.render_widget(title, chunks[0]);

  let widget = match app.current_queue {
    None => {
      Paragraph::new(
        Line::from(Span::styled("Nothing to learn", Style::default().fg(Color::Yellow))).centered())
    },
    Some(queue) => {
      if let Some(card) = app.deck.queues[queue].get_next_card() {
        let front = Span::styled(card.front.to_string(), Style::default());
        Paragraph::new(Line::from(front).centered())
          .block(Block::default()
            .borders(Borders::ALL)
          )
          .wrap(Wrap{trim: false}) 
      } else {
      Paragraph::new(
        Line::from(Span::styled("Something went wrong", Style::default().fg(Color::Red))).centered())
      }
    }
  };

  f.render_widget(
    widget,
    chunks[1]
  );

  match (&app.current_screen, app.current_queue) {
    (CurrentScreen::Checking, Some(queue)) => {
      if let Some(card) = app.deck.queues[queue].get_next_card() {
        let back = Span::styled(card.back.to_string(), Style::default());
        f.render_widget(
          Paragraph::new(Line::from(back).centered())
            .block(Block::default()
              .borders(Borders::ALL)
            )
            .wrap(Wrap{trim: false}), 
          chunks[2]
        );
      }
    }
    _ => {}
  }

  let current_keys_hint = match (&app.current_screen, app.current_queue) {
    (_, None) => Span::styled("(q) quit", Style::default()),
    (CurrentScreen::Asking, Some(_)) => Span::styled("Do you know this?", Style::default()),
    (CurrentScreen::Checking, Some(_)) => Span::styled("Did you know this? (y) yes (n) no", Style::default()),
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
    .split(chunks[3]);

  f.render_widget(key_notes_footer, footer_chunks[0]);
  f.render_widget(queues_footer, footer_chunks[1]);
}

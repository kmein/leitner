use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
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

    let title = Paragraph::new(Line::styled(
        &app.file_name,
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);

    let widget = match app.current_queue {
        None => {
            let message = if app.deck.can_refill() {
                format!(
                    "There are {} new cards available. Do you want to refill?",
                    app.deck.stash_size()
                )
            } else {
                "Nothing to learn.".to_string()
            };
            Paragraph::new(
                Line::styled(message.clone(), Style::default().fg(Color::Yellow)).centered(),
            )
        }
        Some(queue) => {
            if let Some(card) = app.deck.queues[queue].get_next_card() {
                Paragraph::new(Line::styled(card.front.to_string(), Style::default()).centered())
                    .block(Block::default().borders(Borders::ALL))
                    .wrap(Wrap { trim: false })
            } else {
                Paragraph::new(
                    Line::styled("Something went wrong", Style::default().fg(Color::Red))
                        .centered(),
                )
            }
        }
    };

    f.render_widget(widget, chunks[1]);

    match (&app.current_screen, app.current_queue) {
        (CurrentScreen::Checking, Some(queue)) => {
            if let Some(card) = app.deck.queues[queue].get_next_card() {
                let back = card.back.to_string();
                f.render_widget(
                    Paragraph::new(Line::styled(back, Style::default()).centered())
                        .block(Block::default().borders(Borders::ALL))
                        .wrap(Wrap { trim: false }),
                    chunks[2],
                );
            }
        }
        _ => {}
    }

    let current_keys_hint = match (&app.current_screen, app.current_queue) {
        (_, None) => {
            if app.deck.can_refill() {
                Span::styled("(r) refill (q) quit", Style::default())
            } else {
                Span::styled("(q) quit", Style::default())
            }
        }
        (CurrentScreen::Asking, Some(_)) => Span::styled("Do you know this?", Style::default()),
        (CurrentScreen::Checking, Some(_)) => {
            Span::styled("Did you know this? (y) yes (n) no", Style::default())
        }
    };

    let queues_footer = Paragraph::new(Line::from(
        app.deck
            .queues
            .iter()
            .enumerate()
            .map(|(index, q)| {
                let style = if app.current_queue == Some(index) {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                };
                Span::styled(format!("{} ", q.cards.len()), style)
            })
            .collect::<Vec<_>>(),
    ))
    .block(Block::default().borders(Borders::ALL));

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[3]);

    f.render_widget(key_notes_footer, footer_chunks[0]);
    f.render_widget(queues_footer, footer_chunks[1]);
}

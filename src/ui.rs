use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen};

fn header_ui(app: &App) -> Paragraph {
    Paragraph::new(Line::styled(
        &app.file_name,
        Style::default().fg(Color::Cyan),
    ))
    .block(Block::bordered())
}

fn card_ui(string: String) -> Paragraph<'static> {
    Paragraph::new(Line::styled(string, Style::default()).centered())
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: false })
}

fn message_ui(string: String, optional_color: Option<Color>) -> Paragraph<'static> {
    let style = match optional_color {
        Some(color) => Style::default().fg(color),
        None => Style::default(),
    };
    Paragraph::new(Line::styled(string, style).centered())
}

fn card_front_ui(app: &App) -> Paragraph {
    match app.current_queue {
        Some(queue) => match app.deck.queues[queue].get_next_card() {
            Some(card) => card_ui(card.front.to_string()),
            None => message_ui("Something went wrong.".to_string(), Some(Color::Red)),
        },
        None if app.deck.can_refill() => {
            let message = format!(
                "There are {} new cards available. Do you want to refill?",
                app.deck.stash_size()
            );
            message_ui(message, Some(Color::Yellow))
        }
        None => message_ui("Nothing to learn.".to_string(), Some(Color::Yellow)),
    }
}

fn current_keys_ui(app: &App) -> Paragraph {
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
    Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL))
}

fn deck_overview_ui(app: &App) -> Paragraph {
    Paragraph::new(Line::from(
        app.deck
            .queues
            .iter()
            .enumerate()
            .map(|(i, q)| {
                let style = match app.current_queue {
                    Some(index) if index == i => Style::default().fg(Color::Cyan),
                    _ => Style::default(),
                };
                Span::styled(format!("{} ", q.cards.len()), style)
            })
            .collect::<Vec<_>>(),
    ))
    .block(Block::bordered())
}

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

    let header = chunks[0];
    let card_front = chunks[1];
    let card_back = chunks[2];
    let footer = chunks[3];

    f.render_widget(header_ui(app), header);
    f.render_widget(card_front_ui(app), card_front);

    if let (CurrentScreen::Checking, Some(queue)) = (&app.current_screen, app.current_queue) {
        if let Some(card) = app.deck.queues[queue].get_next_card() {
            f.render_widget(card_ui(card.back.to_string()), card_back);
        }
    }

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(footer);

    f.render_widget(current_keys_ui(&app), footer_chunks[0]);
    f.render_widget(deck_overview_ui(&app), footer_chunks[1]);
}

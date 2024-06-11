use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
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
    let lines: Vec<Line> = string
        .lines()
        .map(|line| Line::styled(line.to_string(), Style::default()).centered())
        .collect();

    Paragraph::new(lines)
        .block(Block::bordered())
        .wrap(Wrap { trim: false })
}

fn message_ui(string: String, optional_color: Option<Color>) -> Paragraph<'static> {
    let style = match optional_color {
        Some(color) => Style::default().fg(color),
        None => Style::default(),
    };
    Paragraph::new(Line::styled(string, style).centered())
}

fn key_info_ui(key: char, gloss: &str) -> Vec<Span> {
    vec![
        Span::styled(format!("({})", key), Style::default().fg(Color::Yellow)),
        Span::styled(format!(" {} ", gloss), Style::default()),
    ]
}

fn current_keys_ui(app: &App) -> Paragraph {
    let mut spans = Vec::new();
    match (&app.current_screen, app.current_queue) {
        (_, None) => {
            if app.deck.can_refill() {
                spans.append(&mut key_info_ui('r', "refill"));
            };
            spans.append(&mut key_info_ui('q', "quit"))
        }
        (CurrentScreen::Asking, Some(_)) => {
            spans.push(Span::styled("Do you know this?", Style::default()))
        }
        (CurrentScreen::Checking, Some(_)) => {
            spans.push(Span::styled("Did you know this? ", Style::default()));
            spans.append(&mut key_info_ui('y', "yes"));
            spans.append(&mut key_info_ui('n', "no"));
        }
    };
    Paragraph::new(Line::from(spans)).block(Block::bordered())
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
    // create the layout
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

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(footer);

    let footer_left = footer_chunks[0];
    let footer_right = footer_chunks[1];

    // render the layout
    f.render_widget(header_ui(app), header);
    f.render_widget(current_keys_ui(&app), footer_left);
    f.render_widget(deck_overview_ui(&app), footer_right);

    match app
        .current_queue
        .and_then(|q| app.deck.queues[q].get_next_card())
    {
        Some(card) => {
            f.render_widget(card_ui(card.front.to_string()), card_front);
            if app.current_screen == CurrentScreen::Checking {
                f.render_widget(card_ui(card.back.to_string()), card_back);
            }
        }
        None if app.deck.can_refill() => {
            let message_string = format!(
                "There are {} new cards available. Do you want to refill?",
                app.deck.stash_size()
            );
            let message = message_ui(message_string, Some(Color::Yellow));
            f.render_widget(message, card_front)
        }
        None => {
            let message = message_ui("Nothing to learn.".to_string(), Some(Color::Yellow));
            f.render_widget(message, card_front)
        }
    }
}

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use super::state::UiState;
use crate::models::message::Message;

pub struct UiRenderer;

impl Default for UiRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl UiRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, frame: &mut Frame, state: &UiState) {
        let chunks = self.create_layout(frame.area());

        self.render_messages(frame, &chunks[0], state);
        self.render_input(frame, &chunks[1], state);
        self.render_status_bar(frame, &chunks[2], state);
    }

    fn create_layout(&self, area: Rect) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(75),
                Constraint::Percentage(20),
                Constraint::Percentage(5),
            ])
            .split(area)
            .to_vec()
    }

    fn render_messages(&self, frame: &mut Frame, area: &Rect, state: &UiState) {
        let visible_height = area.height.saturating_sub(2) as usize; // учет границ
        let visible_messages = state.get_visible_messages(visible_height);

        let items: Vec<ListItem> = visible_messages
            .iter()
            .map(|msg| self.create_message_item(msg, &state.username))
            .collect();

        let title = format!("Чат ({} сообщений)", state.messages.len());
        let messages_widget = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::Cyan)),
        );

        frame.render_widget(messages_widget, *area);
    }

    fn create_message_item(&self, msg: &Message, current_user: &str) -> ListItem {
        let timestamp = self.format_timestamp(&msg.timestamp);
        let is_own_message = msg.sender == current_user;

        let sender_style = if is_own_message {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        };

        let content_style = if is_own_message {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(Color::Gray)
        };

        let timestamp_str = format!("[{}] ", timestamp);
        let sender_str = format!("{}: ", msg.sender);

        ListItem::new(Line::from(vec![
            Span::styled(timestamp_str, Style::default().fg(Color::DarkGray)),
            Span::styled(sender_str, sender_style),
            Span::styled(msg.content.clone(), content_style),
        ]))
    }

    fn render_input(&self, frame: &mut Frame, area: &Rect, state: &UiState) {
        let input_widget = Paragraph::new(state.get_input())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Введите сообщение (ESC - выход, ↑↓ - прокрутка)")
                    .border_style(Style::default().fg(Color::Green)),
            )
            .style(Style::default().fg(Color::White));

        frame.render_widget(input_widget, *area);
    }

    fn render_status_bar(&self, frame: &mut Frame, area: &Rect, state: &UiState) {
        let status_text = format!(
            "Пользователь: {} | Сообщений: {} | Ctrl+C - выход",
            state.username,
            state.messages.len()
        );

        let status_widget = Paragraph::new(status_text)
            .style(Style::default().bg(Color::DarkGray).fg(Color::White))
            .block(Block::default());

        frame.render_widget(status_widget, *area);
    }

    fn format_timestamp(&self, timestamp: &str) -> String {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc2822(timestamp) {
            dt.format("%H:%M:%S").to_string()
        } else {
            "??:??:??".to_string()
        }
    }
}

use super::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    text::Text,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

pub fn render_ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(f.area());

    let items: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| {
            ListItem::new(Text::raw(format!(
                "{} ({}): {}",
                m.sender, m.timestamp, m.content
            )))
        })
        .collect();
    let mut list_state = ListState::default();
    list_state.select(if app.messages.is_empty() {
        None
    } else {
        Some(app.messages.len() - 1)
    });
    let list = List::new(items)
        .block(Block::default().title("Сообщения").borders(Borders::ALL))
        .highlight_symbol(">> ");
    f.render_stateful_widget(list, chunks[0], &mut list_state);

    let input_block = Paragraph::new(app.input.as_str())
        .block(Block::default().title("Ввод").borders(Borders::ALL));
    f.render_widget(input_block, chunks[1]);
}

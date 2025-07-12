use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use std::io;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::models::message::Message;

pub async fn run_ui(
    mut user_rx: Receiver<Message>,
    net_tx: Sender<Message>,
    username: String,
    token: String,
) -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut messages: Vec<Message> = vec![]; // список сообщений

    //состояние интерфейса!!!
    let mut input = String::new();
    loop {
        // рисовашка интерфейса
        terminal.draw(|f| {
            //чанки окна
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(80), // окно сообщений
                    Constraint::Percentage(20), //      ввода
                ])
                .split(f.area());

            // окно сообщений
            let items: Vec<ListItem> = messages
                .iter()
                .map(|msg| {
                    ListItem::new(Line::from(vec![
                        Span::styled(
                            format!("[{}] {}: ", msg.timestamp, msg.sender),
                            Style::default().fg(Color::Yellow),
                        ),
                        Span::raw(&msg.content),
                    ]))
                })
                .collect();
            let messages_widget =
                List::new(items).block(Block::default().borders(Borders::ALL).title("Чат"));
            f.render_widget(messages_widget, chunks[0]);

            // ввод
            let input_widget = Paragraph::new(input.as_str())
                .block(Block::default().borders(Borders::ALL).title("Ввод"));
            f.render_widget(input_widget, chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        if !input.is_empty() {
                            let msg = Message::new(
                                username.clone(),
                                input.clone(),
                                chrono::Utc::now().to_rfc2822(),
                                token.clone(),
                            );
                            if let Err(e) = net_tx.send(msg.clone()).await {
                                eprintln!("Ошибка отправки: {}", e);
                            }
                            messages.push(msg);
                            input.clear();
                        }
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        }

        while let Ok(msg) = user_rx.try_recv() {
            messages.push(msg);
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}

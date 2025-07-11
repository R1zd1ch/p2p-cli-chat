use crate::models::message::Message;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use tokio::sync::mpsc;

pub mod event;
pub mod render;

pub struct App {
    pub messages: Vec<Message>,
    pub input: String,
    pub sender_tx: mpsc::Sender<Message>,
}

impl App {
    pub fn new(sender_tx: mpsc::Sender<Message>) -> Self {
        App {
            messages: Vec::new(),
            input: String::new(),
            sender_tx,
        }
    }
}

pub async fn run_ui(
    sender_tx: mpsc::Sender<Message>,
    mut receiver_rx: mpsc::Receiver<Message>,
) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new(sender_tx);

    loop {
        terminal.draw(|f| render::render_ui(f, &app))?;

        if let Some(action) = event::handle_event(&mut app)? {
            match action {
                event::Action::Quit => break,
                event::Action::Send => {
                    let message = Message {
                        sender: "User".to_string(),
                        content: app.input.clone(),
                        timestamp: chrono::Local::now().to_rfc3339(),
                    };
                    app.messages.push(message.clone());
                    if let Err(e) = app.sender_tx.send(message).await {
                        eprintln!("Ошибка отправки сообщения в канал: {}", e);
                    }
                    app.input.clear();
                }
            }
        }

        while let Ok(message) = receiver_rx.try_recv() {
            app.messages.push(message);
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

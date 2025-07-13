pub mod events;
pub mod renderer;
pub mod state;

pub use events::EventHandler;
pub use renderer::UiRenderer;
pub use state::UiState;

use crossterm::{
    event::{self},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use std::io;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::models::message::Message;

pub async fn run_ui(
    mut user_rx: Receiver<Message>,
    net_tx: Sender<Message>,
    username: String,
    token: String,
) -> io::Result<()> {
    // инит терма
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // инит компонентов интерфейса
    let mut state = UiState::new(username, token);
    let event_handler = EventHandler::new();
    let renderer = UiRenderer::new();

    // цикл аппки
    loop {
        // рисуем уи
        terminal.draw(|frame| {
            renderer.render(frame, &state);
        })?;

        // обрабатываем события
        if event::poll(std::time::Duration::from_millis(100))? {
            let event = event::read()?;

            if let Err(e) = event_handler.handle_crossterm_event(event, &mut state, &net_tx) {
                eprintln!("Ошибка обработки события: {}", e);
            }
        }

        // чекаем выйдём мы или нет
        if state.should_quit() {
            break;
        }

        // обрабатываем входящие сообщения
        while let Ok(msg) = user_rx.try_recv() {
            state.add_message(msg);
        }
    }

    // чистим терминал
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}

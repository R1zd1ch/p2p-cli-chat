use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tokio::sync::mpsc::Sender;

use super::state::UiState;
use crate::models::message::Message;

pub enum UiEvent {
    SendMessage(Message),
    Quit,
    ScrollUp,
    ScrollDown,
    ScrollToBottom,
}

pub struct EventHandler;

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle_key_event(
        &self,
        key_event: KeyEvent,
        state: &mut UiState,
        net_tx: &Sender<Message>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.process_key_event(key_event, state) {
            Some(UiEvent::SendMessage(message)) => {
                if let Err(e) = net_tx.send(message.clone()).await {
                    eprintln!("Ошибка отправки сообщения: {}", e);
                } else {
                    state.add_message(message);
                    state.clear_input();
                }
            }
            Some(UiEvent::Quit) => {
                state.quit();
            }
            Some(UiEvent::ScrollUp) => {
                state.scroll_up();
            }
            Some(UiEvent::ScrollDown) => {
                state.scroll_down();
            }
            Some(UiEvent::ScrollToBottom) => {
                state.scroll_to_bottom();
            }
            None => {}
        }
        Ok(())
    }

    fn process_key_event(&self, key_event: KeyEvent, state: &mut UiState) -> Option<UiEvent> {
        match key_event.code {
            KeyCode::Char(c) => {
                if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    self.handle_control_char(c, state)
                } else {
                    state.push_char(c);
                    None
                }
            }
            KeyCode::Backspace => {
                state.pop_char();
                None
            }
            KeyCode::Enter => {
                if !state.is_input_empty() {
                    Some(UiEvent::SendMessage(state.create_message()))
                } else {
                    None
                }
            }
            KeyCode::Esc => Some(UiEvent::Quit),
            KeyCode::Up => Some(UiEvent::ScrollUp),
            KeyCode::Down => Some(UiEvent::ScrollDown),
            KeyCode::End => Some(UiEvent::ScrollToBottom),
            KeyCode::PageUp => {
                // вверх
                for _ in 0..5 {
                    state.scroll_up();
                }
                None
            }
            KeyCode::PageDown => {
                // вниз
                for _ in 0..5 {
                    state.scroll_down();
                }
                None
            }
            _ => None,
        }
    }

    fn handle_control_char(&self, c: char, state: &mut UiState) -> Option<UiEvent> {
        match c {
            'c' => Some(UiEvent::Quit), // Ctrl+C для выхода
            'l' => {
                // Ctrl+L пока просто скролл вниз
                Some(UiEvent::ScrollToBottom)
            }
            'u' => {
                // Ctrl+U для очистки строки ввода
                state.clear_input();
                None
            }
            _ => None,
        }
    }

    pub fn handle_crossterm_event(
        &self,
        event: Event,
        state: &mut UiState,
        net_tx: &Sender<Message>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            Event::Key(key_event) => tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current()
                    .block_on(self.handle_key_event(key_event, state, net_tx))
            }),
            Event::Resize(_, _) => Ok(()),
            _ => Ok(()),
        }
    }
}

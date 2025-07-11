use super::App;
use crossterm::event::{self, Event, KeyCode};
use std::io;

pub enum Action {
    Quit,
    Send,
}

pub fn handle_event(app: &mut App) -> io::Result<Option<Action>> {
    if event::poll(std::time::Duration::from_millis(10))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(Some(Action::Quit)),
                KeyCode::Enter => return Ok(Some(Action::Send)),
                KeyCode::Char(c) => {
                    app.input.push(c);
                    return Ok(None);
                }
                KeyCode::Backspace => {
                    app.input.pop();
                    return Ok(None);
                }
                _ => {}
            }
        }
    }
    Ok(None)
}

use crate::models::message::Message;

#[derive(Debug, Clone)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug, Clone)]
pub enum AppState {
    Running,
    Quitting,
}

pub struct UiState {
    pub messages: Vec<Message>,
    pub input: String,
    pub input_mode: InputMode,
    pub app_state: AppState,
    pub username: String,
    pub token: String,
    pub scroll_offset: usize,
    pub max_messages: usize,
}

impl UiState {
    pub fn new(username: String, token: String) -> Self {
        Self {
            messages: Vec::new(),
            input: String::new(),
            input_mode: InputMode::Normal,
            app_state: AppState::Running,
            username,
            token,
            scroll_offset: 0,
            max_messages: 1000,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);

        if self.messages.len() > self.max_messages {
            self.messages.remove(0);
        }

        self.scroll_to_bottom();
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
    }

    pub fn push_char(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn pop_char(&mut self) {
        self.input.pop();
    }

    pub fn is_input_empty(&self) -> bool {
        self.input.is_empty()
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }

    pub fn create_message(&self) -> Message {
        Message::new(
            self.username.clone(),
            self.input.clone(),
            chrono::Utc::now().to_rfc2822(),
            self.token.clone(),
        )
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        let max_scroll = self.messages.len().saturating_sub(1);
        if self.scroll_offset < max_scroll {
            self.scroll_offset += 1;
        }
    }

    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = self.messages.len().saturating_sub(1);
    }

    pub fn quit(&mut self) {
        self.app_state = AppState::Quitting;
    }

    pub fn should_quit(&self) -> bool {
        matches!(self.app_state, AppState::Quitting)
    }

    pub fn get_visible_messages(&self, height: usize) -> &[Message] {
        let start = self.scroll_offset.saturating_sub(height.saturating_sub(1));
        let end = (start + height).min(self.messages.len());
        &self.messages[start..end]
    }
}

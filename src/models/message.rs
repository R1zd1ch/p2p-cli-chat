use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub id: String,
    pub sender: String,
    pub content: String,
    pub timestamp: String,
    pub token: String,
}

impl Message {
    pub fn new(sender: String, content: String, timestamp: String, token: String) -> Self {
        Message {
            id: uuid::Uuid::new_v4().to_string(),
            sender,
            content,
            timestamp,
            token,
        }
    }
}

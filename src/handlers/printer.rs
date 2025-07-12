use tokio::sync::mpsc::Receiver;

use crate::models::message::Message;

pub async fn print_messages(mut user_rx: Receiver<Message>) {
    while let Some(message) = user_rx.recv().await {
        println!(
            "[{}] {}: {}",
            message.timestamp, message.sender, message.content
        );
    }
}

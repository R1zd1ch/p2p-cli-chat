use chrono::Utc;
use std::io::{self, BufRead, Write};
use tokio::sync::mpsc::Sender;

use crate::models::message::Message;

pub async fn handle_input(username: String, token: String, net_tx: Sender<Message>) {
    let stdin = io::stdin();
    let handle = stdin.lock();

    println!("Введите сообщение (Ctrl+C для выхода)");
    print!("> ");
    io::stdout().flush().unwrap();

    for line in handle.lines() {
        let content = match line {
            Ok(line) => line.trim().to_string(),
            Err(e) => {
                eprintln!("Ошибка чтения: {}", e);
                continue;
            }
        };

        if content.is_empty() {
            print!("> ");
            io::stdout().flush().unwrap();
            continue;
        }

        let msg = Message::new(
            username.clone(),
            content,
            Utc::now().to_rfc2822(),
            token.clone(),
        );

        if let Err(e) = net_tx.send(msg).await {
            eprintln!("Ошибка отправки: {}", e);
        }

        print!("> ");
        io::stdout().flush().unwrap();
    }
}

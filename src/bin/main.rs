use chrono::Utc;
use std::io::{self, BufRead, Write};
use tokio::sync::mpsc;

use p2p_cli_chat::config::Config;
use p2p_cli_chat::models::message::Message;
use p2p_cli_chat::network::{client, server};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let server_addr = args.get(1).cloned().unwrap_or("127.0.0.1:8080".to_string());
    let peer_addr = args.get(2).cloned().unwrap_or("127.0.0.1:8081".to_string());
    let username = args.get(3).cloned().unwrap_or("Anonymous".to_string());
    let token = args.get(4).cloned().unwrap_or("default_token".to_string());

    let config = Config {
        server_addr: server_addr.clone(),
        token: token.clone(),
    };

    let (sender_tx, sender_rx) = mpsc::channel::<Message>(100);
    let (receiver_tx, mut receiver_rx) = mpsc::channel::<Message>(100);
    let (server_ready_tx, server_ready_rx) = tokio::sync::oneshot::channel::<()>();
    let (client_ready_tx, client_ready_rx) = tokio::sync::oneshot::channel::<()>();

    tokio::spawn(server::run(
        config.clone(),
        receiver_tx.clone(),
        server_ready_tx,
    ));

    tokio::spawn(client::connect_to_peer(
        peer_addr.to_owned(),
        receiver_tx.clone(),
        sender_rx,
        config.token.clone(),
        client_ready_tx,
    ));

    tokio::spawn(async move {
        while let Some(message) = receiver_rx.recv().await {
            println!(
                "[{}] {}: {}",
                message.timestamp, message.sender, message.content
            );
        }
    });

    let _ = server_ready_rx.await;
    let _ = client_ready_rx.await;
    let stdin = io::stdin();
    let handle = stdin.lock();
    println!("Введите сообщения (Ctrl+C для выхода)");
    print!("> ");
    io::stdout().flush().unwrap();
    for line in handle.lines() {
        let content = match line {
            Ok(line) => line.trim().to_string(),
            Err(e) => {
                eprintln!("Ошибка чтения ввода: {}", e);
                continue;
            }
        };
        if content.is_empty() {
            print!("> ");
            io::stdout().flush().unwrap();
            continue;
        }
        let message = Message::new(
            username.clone(),
            content,
            Utc::now().to_rfc2822(),
            token.clone(),
        );

        //Тут мы отправляем в канал и sender_rx получает его и потом отправляет в receiver_tx
        if let Err(e) = sender_tx.send(message).await {
            eprintln!("Ошибка отправки сообщения: {}", e);
        }
        print!("> ");
        io::stdout().flush().unwrap();
    }
}

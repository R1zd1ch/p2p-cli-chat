use std::time::Duration;

use crate::{config::SharedConfig, models::message::Message, network::message};
use futures_util::{SinkExt, StreamExt};
use http::Uri;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot};
use tokio::time;
use tokio_websockets::{ClientBuilder, MaybeTlsStream, WebSocketStream};

pub struct PeerClient {
    config: SharedConfig,
    user_tx: mpsc::Sender<Message>,
    net_rx: mpsc::Receiver<Message>,
    client_ready_tx: Option<oneshot::Sender<()>>,
}

impl PeerClient {
    pub fn new(
        config: SharedConfig,
        user_tx: mpsc::Sender<Message>,
        net_rx: mpsc::Receiver<Message>,
        client_ready_tx: oneshot::Sender<()>,
    ) -> Self {
        Self {
            config,
            user_tx,
            net_rx,
            client_ready_tx: Some(client_ready_tx),
        }
    }

    pub async fn run(&mut self) {
        let (addr, token) = (
            self.config.peer_addr().to_string(),
            self.config.token().to_string(),
        );

        loop {
            let uri: Uri = match format!("ws://{}", addr).parse() {
                Ok(uri) => uri,
                Err(e) => {
                    eprintln!("Ошибка парсинга URI: {}", e);
                    tokio::time::sleep(time::Duration::from_secs(5)).await;
                    continue;
                }
            };

            match ClientBuilder::from_uri(uri).connect().await {
                Ok((ws_stream, _response)) => {
                    if let Some(tx) = self.client_ready_tx.take() {
                        let _ = tx.send(());
                    }

                    if let Err(e) = self.handle_connection(ws_stream, token.clone()).await {
                        eprintln!("Ошибка в обработке соединения: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Ошибка соединения с {}: {}. Повтор через 5 сек", addr, e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    async fn handle_connection(
        &mut self,
        ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
        token: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (mut sink, mut stream) = ws_stream.split();

        // Отправляем сообщение аутентификации
        let auth_message = Message::new(
            "system".to_string(),
            "auth".to_string(),
            chrono::Utc::now().to_rfc3339(),
            token.clone(),
        );
        message::send_message(&mut sink, &auth_message).await;

        // Запускаем задачу для чтения сообщений
        let user_tx = self.user_tx.clone();
        let addr = self.config.peer_addr().to_string();
        let rx_task = tokio::spawn(async move {
            while let Some(Ok(msg)) = stream.next().await {
                if let Some(text) = msg.as_text() {
                    if let Ok(message) = serde_json::from_str::<Message>(text) {
                        if let Err(e) = user_tx.send(message).await {
                            eprintln!("Ошибка отправки в канал: {}", e);
                        }
                    }
                }
            }
            eprintln!("Соединение с {} закрыто", addr);
        });

        // Обрабатываем отправку сообщений
        while let Some(mut message) = self.net_rx.recv().await {
            message.token = token.clone();
            message::send_message(&mut sink, &message).await;
        }

        // Закрываем соединение
        rx_task.abort();
        sink.close().await?;
        Ok(())
    }
}

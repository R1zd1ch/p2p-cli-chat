use crate::{config::SharedConfig, models::message::Message, network::message};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::{mpsc, oneshot};
use tokio_websockets::ClientBuilder;

pub async fn connect_to_peer(
    config: SharedConfig,
    user_tx: mpsc::Sender<Message>,
    mut net_rx: mpsc::Receiver<Message>,
    client_ready_tx: oneshot::Sender<()>,
) {
    // println!("Подключаемся к пиру на {}", addr);
    let (addr, token) = (config.peer_addr().to_string(), config.token().to_string());

    let mut maybe_ready_tx = Some(client_ready_tx);

    loop {
        let uri = match format!("ws://{}", addr).parse() {
            Ok(uri) => uri,
            Err(e) => {
                eprintln!("Ошибка парсинга URI: {}", e);
                continue;
            }
        };

        match ClientBuilder::from_uri(uri).connect().await {
            Ok((ws_stream, _response)) => {
                // println!("Подключено к {}", addr_clone);
                if let Some(tx) = maybe_ready_tx.take() {
                    let _ = tx.send(());
                }

                let (mut sink, mut stream) = ws_stream.split();
                let auth_message = Message::new(
                    "system".to_string(),
                    "auth".to_string(),
                    chrono::Utc::now().to_rfc3339(),
                    token.clone(),
                );
                message::send_message(&mut sink, &auth_message).await;

                let rx_task = {
                    let user_tx = user_tx.clone();
                    let addr_clone_for_task = addr.clone();
                    tokio::spawn(async move {
                        while let Some(Ok(msg)) = stream.next().await {
                            if let Some(text) = msg.as_text() {
                                if let Ok(message) = serde_json::from_str::<Message>(text) {
                                    if let Err(e) = user_tx.send(message).await {
                                        eprintln!("Ошибка отправки в канал: {}", e);
                                    }
                                }
                            }
                        }

                        eprintln!("Соединение с {} закрыто", addr_clone_for_task);
                    })
                };

                while let Some(mut message) = net_rx.recv().await {
                    message.token = token.clone();
                    message::send_message(&mut sink, &message).await;
                }

                //закрытие соединения
                rx_task.abort();
                if let Err(e) = sink.close().await {
                    eprintln!("Ошибка закрытия соединения: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Ошибка соединения с {}: {}. Повтор через 5 сек", addr, e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }
}

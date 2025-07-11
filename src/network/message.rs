use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use serde_json;
use tokio::net::TcpStream;
use tokio_websockets::{MaybeTlsStream, Message as WsMessage, WebSocketStream};

use crate::models::message::Message;

pub async fn send_message(
    sink: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>,
    msg: &Message,
) {
    let serialized = serde_json::to_string(msg).unwrap();
    if let Err(e) = sink.send(WsMessage::text(serialized)).await {
        eprintln!("Ошибка отправки сообщения: {}", e);
    }
}

pub async fn receive_messages(
    mut stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    mut sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>,
    receiver_tx: tokio::sync::mpsc::Sender<Message>,
) {
    while let Some(Ok(msg)) = stream.next().await {
        if let Some(text) = msg.as_text() {
            if let Ok(message) = serde_json::from_str::<Message>(text) {
                if let Err(e) = receiver_tx.send(message).await {
                    eprintln!("Ошибка отправки сообщения в канал: {}", e);
                }
            }
        }
    }

    handle_connection_close(&mut sink).await;
}

async fn handle_connection_close(
    sink: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>,
) {
    if let Err(e) = sink.close().await {
        eprintln!("Ошибка при закрытии соединения: {}", e);
    }
}

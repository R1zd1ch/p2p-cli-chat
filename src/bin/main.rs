use clap::Parser;
use p2p_cli_chat::config::cli::CliArgs;
use p2p_cli_chat::handlers::ui::run_ui;
use tokio::sync::mpsc;

use p2p_cli_chat::config::Config;
use p2p_cli_chat::models::message::Message;
use p2p_cli_chat::network::{client, server};

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    let config = Config::from_args(&args);

    let (net_tx, net_rx) = mpsc::channel::<Message>(100);
    let (user_tx, user_rx) = mpsc::channel::<Message>(100);

    let (server_ready_tx, server_ready_rx) = tokio::sync::oneshot::channel::<()>();
    let (client_ready_tx, client_ready_rx) = tokio::sync::oneshot::channel::<()>();

    tokio::spawn(server::run(
        config.clone(),
        user_tx.clone(),
        server_ready_tx,
    ));

    tokio::spawn(client::connect_to_peer(
        config.peer_addr.to_owned(),
        user_tx.clone(),
        net_rx,
        config.token.clone(),
        client_ready_tx,
    ));

    //ждём старта серва и клиента перед вводом сообщений
    let _ = (server_ready_rx.await, client_ready_rx.await);
    run_ui(
        user_rx,
        net_tx.clone(),
        config.username.clone(),
        config.token.clone(),
    )
    .await
    .unwrap();
}

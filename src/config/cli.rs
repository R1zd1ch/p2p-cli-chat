use clap::{Parser, command};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(default_value = "127.0.0.1:8080")]
    pub server_addr: String,

    #[arg(default_value = "127.0.0.1:8081")]
    pub peer_addr: String,

    #[arg(default_value = "Anonymous")]
    pub username: String,

    #[arg(default_value = "default_token")]
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub server_addr: String,
    pub token: String,
    pub peer_addr: String,
    pub username: String,
}

impl Config {
    pub fn new(server_addr: String, token: String, peer_addr: String, username: String) -> Self {
        Self {
            server_addr,
            token,
            peer_addr,
            username,
        }
    }
}

pub mod cli;

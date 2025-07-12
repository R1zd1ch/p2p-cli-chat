use std::sync::Arc;

pub mod cli;

pub trait ConfigProvider {
    fn server_addr(&self) -> &str;
    fn token(&self) -> &str;
    fn peer_addr(&self) -> &str;
    fn username(&self) -> &str;
}

pub type SharedConfig = Arc<dyn ConfigProvider + Send + Sync>;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_addr: String,
    pub token: String,
    pub peer_addr: String,
    pub username: String,
}

impl ConfigProvider for Config {
    fn server_addr(&self) -> &str {
        &self.server_addr
    }
    fn token(&self) -> &str {
        &self.token
    }
    fn peer_addr(&self) -> &str {
        &self.peer_addr
    }
    fn username(&self) -> &str {
        &self.username
    }
}

impl Config {
    pub fn from_args(args: &cli::CliArgs) -> Self {
        Config {
            server_addr: args.server_addr.clone(),
            token: args.token.clone(),
            peer_addr: args.peer_addr.clone(),
            username: args.username.clone(),
        }
    }
}

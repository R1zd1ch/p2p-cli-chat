#[derive(Debug, Clone)]
pub struct Config {
    pub server_addr: String,
    pub token: String,
}

impl Config {
    pub fn default_conf() -> Self {
        Self {
            server_addr: "127.0.0.1:8080".to_string(),
            token: "default_token".to_string(),
        }
    }
}

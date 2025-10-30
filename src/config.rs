use std::net::SocketAddr;

pub struct AppConfig {
    pub database_url: String,
    pub qdrant_url: String,
    pub server_addr: SocketAddr,
    pub openai_api_key: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            qdrant_url: std::env::var("QDRANT_URL")?,
            server_addr: std::env::var("SERVER_ADDR")?.parse()?,
            openai_api_key: std::env::var("OPENAI_API_KEY")?,
        })
    }
}

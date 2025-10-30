use qdrant_client::prelude::*;

pub async fn init_client(url: &str) -> anyhow::Result<QdrantClient> {
    let config = QdrantClientConfig::from_url(url);
    Ok(QdrantClient::new(Some(config))?)
}



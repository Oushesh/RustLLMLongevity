use axum::{Router, routing::{get, post}, Extension};
use crate::routes::{chat::chat_handler, health::health_check, search::search_handler};
use prisma_client_rust::PrismaClient;
use qdrant_client::QdrantClient;
use std::sync::Arc;

pub async fn build_router(prisma: PrismaClient, qdrant: QdrantClient) -> Router {
    let prisma = Arc::new(prisma);
    let qdrant = Arc::new(qdrant);

    Router::new()
        .route("/health", get(health_check))
        .route("/chat", post(chat_handler))
        .route("/search", post(search_handler))
        .layer(Extension(prisma))
        .layer(Extension(qdrant))
}

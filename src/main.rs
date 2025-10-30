use tracing_subscriber::EnvFilter; 


//mod is like import modules in python
mod app; 
mod config;
mod db; 
mod qdrant; 
mod routes; 
mod utils; 

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = config::AppConfig::from_env()?;

    // Initialize Prisma client
    let prisma = db::init_prisma().await?;

    // Initialize Qdrant
    let qdrant = qdrant::init_client(&config.qdrant_url).await?;

    // Build app router
    let app = app::build_router(prisma, qdrant).await;

    tracing::info!("🚀 Server running on {}", config.server_addr);
    axum::Server::bind(&config.server_addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}




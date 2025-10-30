use prisma_client_rust::PrismaClient;
use prisma_client_rust::NewClient;

pub async fn init_prisma() -> anyhow::Result<PrismaClient> {
    let client = NewClient::new().await?;
    Ok(client)
}

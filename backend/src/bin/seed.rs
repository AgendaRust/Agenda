use backend::{db, seeds::DatabaseSeeder};
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    println!("🌱 Conectando ao banco de dados...");
    
    // Usar a mesma configuração do main.rs
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://database.db".to_string());
    
    let db = sea_orm::Database::connect(&database_url).await?;
    
    println!("🚀 Executando seeds...");
    DatabaseSeeder::run(&db).await?;
    
    println!("✅ Processo concluído!");
    Ok(())
}
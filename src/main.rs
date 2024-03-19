use sqlx::mysql::MySqlPoolOptions;
use std::env;

#[tokio::main]
async fn main() {
    // Read environment variables and generate URL
    dotenv::dotenv().expect("Fialed to read .env file");
    let database_url = env::var("DATABASE_URL").expect("DABASE_URL must be set");

    // Generate DB connection
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap_or_else(|_| panic!("Cannot connect to the database"));

    // Gererate transaction
    let tx = pool.begin().await.expect("transaction error.");

    // Execute SQL query
    let rows = sqlx::query!("SELECT * FROM Cars")
        .fetch_all(&pool)
        .await
        .expect("Failed to db fetch");

    // transaction commit
    let _ = tx.commit().await.unwrap_or_else(|e| {
        println!("{:?}", e);
    });

    // Print result
    for row in rows {
        println!("Value: {:?}", row.model.unwrap());
    }
}

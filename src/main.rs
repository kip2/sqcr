use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySql;
use sqlx::Pool;
use sqlx::Row;
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

    let query = "SELECT * FROM Cars".to_string();

    execute_query(&pool, &query).await;
}

async fn execute_query(db: &Pool<MySql>, query: &str) {
    // Gererate transaction
    let tx = db.begin().await.expect("transaction error.");

    // Execute SQL query
    let rows = sqlx::query(query)
        .fetch_all(db)
        .await
        .expect("Failed to db fetch");

    // transaction commit
    let _ = tx.commit().await.unwrap_or_else(|e| {
        println!("{:?}", e);
    });

    // Print result
    for row in rows {
        println!("Value: {:?}", row.get::<String, _>("model"));
    }
}

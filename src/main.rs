use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool, Row};
use std::{env, error::Error, fs};

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

    // SQL query file path
    let path = "./migrations/query.sql";

    // Read SQL queries
    let queries = read_sql_file(&path).unwrap();

    execute_query(&pool, queries).await;
}

fn read_sql_file(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    let queries = contents
        .split(';')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
        .collect();
    Ok(queries)
}

async fn execute_query(db: &Pool<MySql>, queries: Vec<String>) {
    // Gererate transaction
    let tx = db.begin().await.expect("transaction error.");

    for query in queries {
        // Execute SQL query
        let result = sqlx::query(&query).fetch_all(db).await;

        match result {
            Ok(rows) => {
                if rows.is_empty() {
                    continue;
                }
                // Print result
                for row in &rows {
                    match row.try_get::<String, _>("model") {
                        Ok(value) => println!("Value: {:?}", value),
                        Err(e) => println!("Error getting model: {}", e),
                    }
                }
            }
            Err(e) => {
                println!("Database query failed: {}", e);
                println!("Failed query: {:?}", &query);
                continue;
            }
        }
    }

    // transaction commit
    let _ = tx.commit().await.unwrap_or_else(|e| {
        println!("{:?}", e);
    });
}

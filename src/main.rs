use sqlx::mysql::MySqlPoolOptions;
use sqlx::{query, MySql, Pool, Row};
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

    // let query = "SELECT * FROM Cars";
    let mut queries: Vec<&str> = Vec::new();
    queries.push("SELECT * FROM Cars");

    queries.push(
        "INSERT INTO Cars (make, model, year, color, price, mileage, transmission, engine, status)
    VALUES ('Toyota', 'Corolla', 2020, 'White', 20000.00, 15000.0, 'Automatic', '1.8L', 'Used')",
    );
    // error case
    queries.push(
        "INSERT INTO Cars (make, model, year, color, price, mileage, transmission, engine, status)
        VALUES ('Toyota', 'Corolla', 'two thousand twenty', 'White', 'twenty thousand', 15000, 'Automatic', '1.8L', 'Used')"
    );

    execute_query(&pool, queries).await;
}

async fn execute_query(db: &Pool<MySql>, queries: Vec<&str>) {
    // Gererate transaction
    let tx = db.begin().await.expect("transaction error.");

    for query in queries {
        // Execute SQL query
        let result = sqlx::query(query).fetch_all(db).await;

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

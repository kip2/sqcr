use std::env;

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    println!("{}", env::var("DATABASE_URL").unwrap());
    println!("Hello, world!");
}

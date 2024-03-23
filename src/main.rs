#[tokio::main]
async fn main() {
    // Retrieve arguments
    let config = match sqcr::get_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    // Execute query
    if let Err(e) = sqcr::run(config).await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

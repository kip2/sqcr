use clap::{Args, Parser};
use std::error::Error;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Config {
    #[arg(short = 'f', long = "filepath", help = "Input file path")]
    filepath: String,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let args = Config::parse();
    Ok(args)
}

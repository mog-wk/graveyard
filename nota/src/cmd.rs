use clap::Parser;
use std::process::exit;

const VERSION: &'static str = "0.0.1";

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short = 'd', long = "debug", default_value = "false")]
    pub debug: bool,
    #[arg(short = 'v', long = "version", default_value = "false")]
    pub version: bool,
    #[arg(short = 'c', long = "config", default_value = None)]
    pub custom_config: Option<String>,
}

pub fn version() {
    println!("\"{}\"", VERSION);
    exit(0);
}

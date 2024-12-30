mod error;
mod logger;

mod cli;
mod tui;

mod app;

mod scraping;

use crate::error::Result;
use std::{io::Write, path::PathBuf};

lazy_static::lazy_static!(
    pub static ref VERSION: &'static str = env!("CARGO_PKG_VERSION");

    pub static ref CRATE_NAME: String = env!("CARGO_CRATE_NAME").to_uppercase();

    pub static ref DATA_DIR: PathBuf = PathBuf::from(std::env::var(format!("{}_DATA_DIR", CRATE_NAME.clone())).ok().unwrap_or(format!("./{}_DATA_DIRECTORY/", CRATE_NAME.clone())));

    pub static ref LOG_FILE: String = format!("{}.log", env!("CARGO_PKG_NAME"));
    pub static ref CACHE_FILE: PathBuf = {
        let path = std::env::var(format!("{}_CACHE", CRATE_NAME.clone()));
        match path {
            Ok(path) => PathBuf::from(path),
            Err(_) => DATA_DIR.join(PathBuf::from(format!("{}_CACHE", CRATE_NAME.clone()))),
        }
    };
    //pub static ref CONFIG_DIR: Option<PathBuf> = Some(PathBuf::from("./test/cache.txt"));
);

fn main() -> Result<()> {
    let args = cli::init();

    dotenv::dotenv()?;

    logger::init(args.debug)?;

    // NOTE: disable get scrapping to not overload servers
    //let scrap_text = scraping::get("https://coincap.io/")?;

    //crude_cache_write(&scrap_text)?;
    let text = crude_cache_read();

    // TODO: parse data

    // TODO: make inline cli

    tui::run()?;

    tracing::info!("exitting");
    Ok(())
}
pub(crate) fn crude_cache_read() -> Result<String> {
    println!("{:?}", CACHE_FILE.clone());
    Ok(std::fs::read_to_string(CACHE_FILE.clone())?)
}

pub(crate) fn crude_cache_write(text: &str) -> Result<()> {
    let cache_file = CACHE_FILE.clone();
    tracing::info!("writing cache in {:?}", &cache_file);

    let mut cache_file = std::fs::File::create(cache_file).unwrap();
    cache_file.write(text.as_bytes()).unwrap();

    Ok(())
}

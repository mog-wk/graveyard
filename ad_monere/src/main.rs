use std::path::PathBuf;

mod cli;
mod common;

mod error;
mod logger;

//#[cfg(target_feature = "tui")]
mod tui;

lazy_static::lazy_static!(
    pub static ref VERSION: &'static str = env!("CARGO_PKG_VERSION");

    pub static ref LOG_DIR: Option<PathBuf> = std::env::var("AD_MONERE_LOGS").ok().map(PathBuf::from);
    pub static ref DEV_LOG: String = format!("{}.dev.log", env!("CARGO_PKG_NAME"));
    pub static ref ENV_LOG: String = format!("{}.log", env!("CARGO_PKG_NAME"));

    pub static ref CACHE_PATH: Option<PathBuf> = std::env::var("AD_MONERE_CACHE").ok().map(PathBuf::from);

    pub static ref CACHE_DEV_PATH: Option<PathBuf> = Some(PathBuf::from("./test/cache.txt"));
);

fn main() -> error::Result<()> {
    let args = cli::init();

    //println!("{:#?}", args);

    dotenv::dotenv()?;

    logger::init(args.get_flag("debug"))?;

    if let Some((cmd, args)) = args.subcommand() {
        match cmd {
            "cli" => cli::run(args)?,
            "tui" => tui::run()?,
            _ => (),
        }
    }

    tracing::info!("exitting");

    Ok(())
}

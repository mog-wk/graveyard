use std::path::Path;
use crate::error::Error as crate_error;

// test dir
const DEV_MUSIC_DIR: &'static str = "src/_dev/msc";

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t=1.to_string())]
    pub test: String,
    #[arg(short, long, default_value_t=DEV_MUSIC_DIR.to_string())]
    pub dir: String,
    #[arg(short, long, default_value_t=false)]
    pub start_ui: bool,
    #[arg(short, long)]
    pub music: Vec<String>,
    #[arg(short, long, default_value_t=false)]
    pub list: bool,
}

pub fn get_default_music_directory() -> Result<&'static Path, crate_error> {
    let p = Path::new(&DEV_MUSIC_DIR);
    if !p.exists() {
        return Err(crate_error::MusicDirectoryCurrupted)
    } 
    Ok(p)
}

#[cfg(test)]
#[path = "../_tests/cli_parser.rs"]
mod tests;

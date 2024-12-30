use crate::error::Result;

use crate::{DATA_DIR, LOG_FILE};

use tracing_subscriber::layer::{Layer, SubscriberExt};
use tracing_subscriber::util::SubscriberInitExt;

pub fn init(debug_mode: bool) -> Result<()> {
    let log_dir = DATA_DIR.clone();

    std::fs::create_dir_all(&log_dir)?;

    let log_file = std::fs::File::create(log_dir.join(LOG_FILE.clone()))?;

    std::env::set_var(
        "RUST_LOG",
        if debug_mode {
            "debug".to_string()
        } else {
            std::env::var("RUST_LOG").unwrap_or("info".to_string())
        },
    );

    let file_sub = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .with_writer(log_file)
        .with_ansi(false)
        .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());

    tracing_subscriber::registry()
        .with(file_sub)
        .with(tracing_error::ErrorLayer::default())
        .init();

    Ok(())
}

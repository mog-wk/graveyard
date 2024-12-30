use std::path::PathBuf;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

use crate::{error::Result, DEV_LOG, ENV_LOG, LOG_DIR};

pub fn init(debug: bool) -> Result<()> {
    let log_dir = LOG_DIR.clone().unwrap_or(PathBuf::from(".").join("log"));
    std::fs::create_dir_all(&log_dir)?;

    let log_path = if debug {
        log_dir.join(DEV_LOG.clone())
    } else {
        log_dir.join(ENV_LOG.clone())
    };

    let log_file = std::fs::File::create(log_path)?;

    std::env::set_var(
        "RUST_LOG",
        if debug {
            "debug".into()
        } else {
            std::env::var("RUST_LOG").unwrap_or("info".to_string())
        },
    );

    let file_sub = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false)
        .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());

    tracing_subscriber::registry()
        .with(file_sub)
        .with(tracing_error::ErrorLayer::default())
        .init();

    tracing::debug!("started logger");

    Ok(())
}

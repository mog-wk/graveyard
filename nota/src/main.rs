mod cmd;
mod conf;
mod error;
mod tui;

use clap::Parser;
use error::Error;

fn main() -> Result<(), Error> {
    let cmd = cmd::Cli::parse();
    std::env::set_var("RUST_BACKTRACE", "1");

    if cmd.version {
        cmd::version();
    }

    if cmd.debug {
        println!("cli: {:?}", cmd);

        println!("size: {:?}", ratatui::crossterm::terminal::size());
    }
    let conf = conf::get(cmd.custom_config)?;
    //println!("{conf:?}");

    tui::init_error_hooks()?;
    let mut terminal = tui::init_terminal()?;
    tui::App::default().run(&mut terminal, conf)?;
    tui::restore_terminal()?;
    Ok(())
}

#[macro_export]
macro_rules! log {
    ($content:expr) => {
        let mut p = std::fs::canonicalize("./").unwrap();
        p.push("log.txt");
        std::fs::write(p, format!("{:?}", $content));
    };
}

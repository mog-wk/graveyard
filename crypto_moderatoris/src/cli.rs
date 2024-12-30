use clap::Parser;
#[derive(Debug, Parser)]
pub(crate) struct Cli {
    #[arg(short = 'd', long = "debug", default_value = "false")]
    pub debug: bool,
}

pub(crate) fn init() -> Cli {
    Cli::parse()
}

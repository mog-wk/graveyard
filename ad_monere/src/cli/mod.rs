use clap::arg;
use clap::command;
use clap::ArgAction;

pub mod app;

pub use self::app::*;

pub fn init() -> clap::ArgMatches {
    command!("")
        .version(*crate::VERSION)
        .author("__")
        .about("check crypt currency prices")
        .arg(arg!(-d --debug "Debug mode").action(ArgAction::SetTrue))
        .subcommand(
            command!("cli")
                .about("returns currency information to stdout")
                .arg(
                    arg!(<TRACKER> "request tracker")
                        .required(false)
                        .default_value("rate.sx"),
                )
                .arg(
                    arg!(<CURRENCY> "request currency")
                        .required(false)
                        .default_value("USD"),
                ),
        )
        .disable_version_flag(true)
        .subcommand(command!("tui"))
        //.arg(arg!(-i --cli <URL> "cli mode").action(ArgAction::SetTrue))
        .get_matches()
}

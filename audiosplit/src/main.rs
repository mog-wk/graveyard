#![allow(unused)]

use std::{
    borrow::Cow,
    fs::{write, File},
    io::{BufRead, Read, Write},
    path::PathBuf,
};

use clap::{arg, command, value_parser, ArgGroup, ArgMatches, Subcommand};
use decoders::format_codec;
use errors::{CliError, Error};
/// cli for audio/video management
use tracing::{debug, error, info, span, Level};
use tracing_subscriber::fmt::format;

mod decoders;
mod errors;
mod utils;

pub const SUPPORTED_EXT: [&'static str; 2] = ["mp4", "ogg"];
pub const SPLIT_DELIMITER: &'static str = " ";

fn main() -> crate::errors::Result<()> {
    let cli = clap::command!("audio man")
        .version("1.0")
        .author("salut")
        .about("Peforms various operations in audio/video files")
        .arg(
            arg!(
                    -d --debug ... "Sets debug mode"
            )
            .action(clap::ArgAction::SetTrue)
            .required(false),
        )
        .arg(
            arg!( <media_file> "Media file to process")
                .required(true)
                .index(1),
        )
        .subcommand(
            command!("split")
                .about("Splits media file")
                //.arg(arg!( -l --list <TIMESTAMP_LIST> "Timestamp list"))
                .arg(arg!( <TIMESTAMPS> "Timestamp input"))
                //.group( ArgGroup::new("split method") .args(["file", "list"]) .multiple(false) .required(true),),
                .arg(
                    arg!( <TYPE> "Describes how the timestamp input will be parsed(")
                        .required(false)
                        .default_value("file"),
                ),
        )
        .subcommand(
            command!("copy")
                .about("Copy media file")
                .arg(arg!( <OUTPUT> "output filename")),
        )
        .get_matches();

    let main_span = if cli.get_flag("debug") {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
        span!(Level::DEBUG, "main")
    } else {
        tracing_subscriber::fmt().with_max_level(Level::INFO).init();
        span!(Level::INFO, "main")
    };

    let _guard = main_span.enter();

    //info!(?cli) && return Err(Error::DEV);

    debug!("staring in DEBUG mode");
    debug!(?cli, "cli");

    // handle main media file
    if let Some(media_file) = cli.get_one::<String>("media_file") {
        let pwd = std::env::current_dir()?;

        let media_filepath = pwd.join(media_file);

        let mut src = std::fs::File::open(&media_filepath)
            .or(Err(CliError::InvalidPath(media_filepath.clone())))?;

        // probe for extentions

        if let Some(ext) = media_filepath.extension() {
            let ext = ext.to_str().expect("failed to convert \'ext\' to str");
            if !SUPPORTED_EXT.contains(&ext) {
                error!("invalid extention provided");
                return Err(CliError::InvalidExtention(ext.to_string()).into());
            }
        } else {
            return Err(CliError::NoExtention(media_filepath).into());
        }

        // handle subcommand
        handle_subcommnad(cli.subcommand(), &pwd);
    }

    info!("Done");
    Ok(())
}

//fn handle_subcommnad<T: Sample + ConvertibleSample>( TODO:
fn handle_subcommnad(cli: Option<(&str, &ArgMatches)>, pwd: &PathBuf) -> Result<(), Error> {
    if let Some(cmd) = cli {
        match cmd {
            ("split", sub) => {
                info!("splitting ... ");
                // handle timestamp file
                let ts_file = pwd.join(sub.get_one::<String>("TIMESTAMPS").unwrap());
                let ts_file = if ts_file.as_path().exists() {
                    ts_file
                } else {
                    error!(?ts_file);
                    return Err(CliError::InvalidPath(ts_file).into());
                };

                match sub.get_one::<String>("TYPE").unwrap().as_str() {
                    "file" => {
                        info!("from file ... ");
                        let ts_file = std::fs::read_to_string(ts_file)?;
                        let (times, titles) = utils::parse_timestamp_file(&ts_file)?;
                        assert_eq!(times.len(), titles.len());
                        let lines_amount = times.len();
                        for i in 0..lines_amount {
                            let start_time = times[i];
                            let end_time = if i == lines_amount - 1 {
                                "TODO: END OF VIDEO TIME"
                            } else {
                                times[i + 1]
                            };

                            let title = titles[i];
                            debug!("({}-{}) {:?}", start_time, end_time, title);

                            // TODO: split audio file
                        }
                    }
                    "list" => {}
                    _ => {}
                }
                todo!();
            }
            ("copy", sub) => {
                let name = sub.get_one::<String>("OUTPUT").unwrap();
                let mut file = File::create(name).unwrap();

                file.write(&[32, 12, 13]);
            }
            // clap handles invalid subcommnads
            _ => {}
        }
    } else {
        error!("invalid command");
        return Err(Error::DEV);
    }

    Ok(())
}

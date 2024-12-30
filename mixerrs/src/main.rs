#![allow(unused)]
mod cli;
mod error;

use clap::Parser;
use colored::Colorize;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() -> Result<(), error::Error> {
    let args = cli::parser::Args::parse();
    println!("{:?}", args);

    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout);

    // TODO windows port ..
    // linux fs
    let music_directory_path = fs::canonicalize("./")?.join(PathBuf::from(args.dir.as_str()));
    let music_directory = fs::read_dir(&music_directory_path).unwrap_or_else(|e| {
        println!(
            "{} \"{:?}\"",
            "Invalid dir location:".red(),
            music_directory_path
        );
        println!("{:?}", e);
        std::process::exit(1);
    });

    // ## list
    if args.list {
        match cli::list(&mut writer, music_directory, &music_directory_path) {
            Ok(_) => println!("{:?}", writer),
            Err(e) => println!("Error listening directory: {}", e),
        };
    }

    // TODO: music player in queue
    for music in args.music {
        println!("m: {:?}", music);
    }
    Ok(())
}

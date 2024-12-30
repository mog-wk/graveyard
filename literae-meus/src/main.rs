#![allow(unused)]
use std::{
    fmt::format,
    fs::{self, read_to_string},
    path::{Path, PathBuf},
    process::exit,
};

mod errors;
mod parsers;
mod tags;
mod utils;

use clap::{error, Parser};
use errors::Error;
use utils::*;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "0")]
    debug: u8,

    #[arg(long, default_value = "raw")]
    parse_mode: utils::ParseType,

    #[arg(required = true)]
    path: String,
}

fn main() -> Result<(), crate::errors::Error> {
    let cli = Args::parse();

    if cli.debug == 4 {
        println!("{cli:?}");
    }

    let md_tag = tags::TagSet::md()?;

    // hardcode path !! // run cargo r in project root directory change to cli in the future
    let book_dir = std::fs::canonicalize("assets/test/").unwrap();
    let src = book_dir.join("src/");

    // TODO: make pdf parameters into struct for external lib
    // write document
    let mut title = utils::get_title(&book_dir).unwrap_or("placeholdertitle".to_string());
    let font_family = genpdf::fonts::from_files("./fonts", "Poppins", None)
        .expect("Failed to process font for pdf");
    let mut doc = genpdf::Document::new(font_family);

    // default decorator
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(8);
    doc.set_page_decorator(decorator);

    // decompose book directory tree
    let summary = read_to_string(src.join("SUMMARY.md")).or(Err(Error::RequiredFileNotFould {
        file: "SUMMARY.md".to_string(),
        default_location: "<book_directory>/src/".to_string(),
    }))?;

    // DEBUG: enumerate
    for (i, line) in summary.lines().enumerate() {
        // skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let line = line.trim_start();
        let line = match line.strip_prefix("- ") {
            Some(v) => v,
            None => line,
        };
        let line = &line[1..line.len() - 1];

        let (section_title, content_path) = line.split_once("](").expect("corrupt line");
        if content_path.starts_with(')') {
            println!("discarting line: {section_title} : {content_path}");
            continue;
        }
        let content_path = &src.join(content_path);

        dbg!(i, section_title, content_path);

        match cli.parse_mode {
            ParseType::Raw => parsers::raw::parse(
                &mut doc,
                &fs::read_to_string(content_path).unwrap(),
                &src,
                &md_tag,
            ),
            ParseType::Latex => parsers::tex::parse(
                &mut doc,
                &fs::read_to_string(content_path).unwrap(),
                &src,
                &md_tag,
            ),
        }
        break;
    }

    doc.set_title(&title);
    title.push_str(".pdf");
    // change title save path -P
    // NOTE: testing with assets path
    doc.render_to_file("assets/".to_string() + &title)
        .expect("failed to write pdf file");
    Ok(())
}

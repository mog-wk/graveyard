//! # Functions related to configuration
use ratatui::crossterm::event::KeyCode;

use crate::error::Error;
use crate::error::Result;
use std::{fs::read_to_string, path::PathBuf};

const CONFIG_DIR: &'static str = "nota/";
const CONFIG_NAME: &'static str = "notarc";
const PARSE_COMMENT: &'static str = "#";

#[derive(Debug)]
pub struct Opts {
    // geral
    pub reload: char,
    pub quit: char,

    // move cursor
    pub cursor_down: KeyCode,
    pub cursor_up: char,
    pub cursor_top: char,
    pub cursor_botton: char,

    // single and selected
    pub delete: char,
    pub open: char,
    pub edit: char,

    // selected
    pub select: char,
    pub yank: char,
    pub bulkrename: char,

    pub notes_dir: PathBuf,
}

impl Default for Opts {
    fn default() -> Self {
        Self {
            reload: 'R',
            quit: 'q',

            cursor_down: KeyCode::Char('j'),
            cursor_up: 'k',
            cursor_top: 'g',
            cursor_botton: 'G',

            delete: 'D',
            open: 'l',
            edit: 'e',

            select: ' ',
            yank: 'y',
            bulkrename: 'B',

            notes_dir: get_default_notes_dir(),
        }
    }
}

#[cfg(target_os = "linux")]
fn get_default_config_path() -> Result<PathBuf> {
    let mut c = dirs::config_dir().expect("unable to get default directory");
    c.push(CONFIG_DIR);
    if !c.exists() {
        std::fs::create_dir(&c)?;
    }
    c.push(CONFIG_NAME);
    if !c.exists() {
        std::fs::File::create(&c)?;
    }
    Ok(c)
}

fn get_custom_config_path(path: PathBuf) -> Result<PathBuf> {
    todo!()
}

/// fails if conf is unable to be parsed
/// parse rules:
/// <Command> <KEY> <VALUE>
/// examples:
/// map j to move cursor down:
/// map j down
fn parse_config(conf: PathBuf) -> Result<Opts> {
    let mut opts: Opts = Opts::default();

    let conf = read_to_string(conf)?;
    for (i, line) in conf.lines().enumerate() {
        // skip comments
        if line.starts_with(PARSE_COMMENT) {
            continue;
        }
        let line = if let Some(content) = line.split_once(PARSE_COMMENT) {
            content.0
        } else {
            line
        };
        let line: Vec<_> = line.split(' ').collect();
        let (command, key, func) = (line[0], line[1], line[2]);

        println!("{line:?}");
        match command {
            "map" => {
                let key = key.chars().next().ok_or(Error::ConfigParseError(i))?;
                match func {
                    "down" => opts.cursor_down = KeyCode::Char(key),
                    "up" => opts.cursor_up = key,
                    "top" => opts.cursor_top = key,
                    "bottom" | "bot" => opts.cursor_botton = key,
                    "yank" => opts.yank = key,
                    "select" => opts.select = key,
                    "bulk" | "bulkrename" => opts.bulkrename = key,
                    "open" | "cat" => opts.open = key,
                    "edit" | "vim" => opts.edit = key,
                    "delete" => opts.delete = key,
                    "reload" => opts.reload = key,
                    "quit" => opts.quit = key,
                    _ => return Err(Error::ConfigParseError(i)),
                }
            }
            "set" => {
                let key = key.into();
                match func {
                    "notes_dir" => opts.notes_dir = key,
                    _ => return Err(Error::ConfigParseError(i)),
                }
            }
            _ => return Err(Error::ConfigParseError(i)),
        }
    }
    Ok(opts)
}

pub fn get_default_notes_dir() -> PathBuf {
    dirs::home_dir().unwrap().join("Media/notes/")
}

/// wrapper
pub fn get(path: Option<impl Into<PathBuf>>) -> Result<Opts> {
    if let Some(path) = path {
        parse_config(get_custom_config_path(path.into())?)
    } else {
        parse_config(get_default_config_path()?)
    }
}

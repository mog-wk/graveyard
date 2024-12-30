use crate::errors::Error;
use crate::errors::Result;
use std::fs::read_to_string;
use std::ops::Range;
use std::ops::RangeBounds;
use std::path::Path;

/// utilitie for managing inline elements like bold, italic, etc
#[derive(Debug)]
pub struct TextFormatBuilder {
    array: Vec<(FormatType, Range<usize>)>,
}

impl TextFormatBuilder {
    pub fn new() -> Self {
        Self { array: Vec::new() }
    }

    pub fn add(mut self, ty: &FormatType, match_arr: Vec<Range<usize>>) -> Self {
        for value in match_arr {
            if !self.has(&value) {
                self.array.push((ty.clone(), value));
            }
        }
        println!("{:?}", self);
        self
    }

    fn has(&self, value: &Range<usize>) -> bool {
        for (_, x) in self.array.iter() {
            if x.contains(&value.start) || x.contains(&value.end) {
                return true;
            }
        }
        false
    }

    pub fn build(mut self) -> Result<Vec<(FormatType, Range<usize>)>> {
        // sort array
        let ln = self.array.len();
        //self.check_array()?;
        format_sort(&mut self.array, 0, ln);
        Ok(self.array)
    }

    /// check for range incompability
    fn check_array(&self) -> Result<()> {
        let ln = self.array.len();
        for i in 0..ln {
            for j in i + 1..ln {
                /* TODO: error handling
                //
                    if !(self.array[i].1.start >= self.array[j].1.end
                        && self.array[i].1.end <= self.array[j].1.start)
                    {
                        println!("{:?}", self);
                        return Err(Error::InvalidFormattingError);
                    }
                */
            }
        }
        Ok(())
    }
}

fn format_sort(arr: &mut Vec<(FormatType, Range<usize>)>, stt: usize, end: usize) {
    if stt >= end {
        // done
        return;
    }
    let p = partition(arr, stt, end);
    format_sort(arr, stt, p - 1);
    format_sort(arr, p, end);
}

fn partition(arr: &mut Vec<(FormatType, Range<usize>)>, stt: usize, end: usize) -> usize {
    let p = end - 1;
    let mut j = stt;
    for i in stt..end - 1 {
        if arr[i].1.start < arr[p].1.start {
            arr.swap(i, j);
            j += 1;
        }
    }

    arr.swap(j, p);
    j + 1
}

#[macro_export]
macro_rules! strip {
    ($st:expr, $pat:expr) => {
        $st.strip_prefix($pat).unwrap().strip_suffix($pat).unwrap()
    };
}

pub fn get_title(book_dir: &Path) -> Option<String> {
    for line in read_to_string(book_dir.join("book.toml")).unwrap().lines() {
        if line.starts_with("title") {
            return Some(
                line.split_once("=")
                    .expect("invalid formatting in book.toml")
                    .1
                    .trim()
                    .to_lowercase()
                    .strip_prefix('\"')?
                    .strip_suffix('\"')?
                    .to_string(),
            );
        }
    }

    None
}

/// Parse the book.toml file into metadata information
pub fn get_metadata(book_dir: &Path) {
    let mut section: &str = &"";

    // Data
    let book_toml = read_to_string(book_dir.join("book.toml")).unwrap();

    for (i, line) in book_toml.lines().enumerate() {
        println!("{i} {line}");

        let mut title = String::new();

        // skips comment
        if line.starts_with("#") {
            continue;
        }
        // new section
        if line.starts_with("[") {
            section = line.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
            continue;
        }
        // parse in section
        if !line.is_empty() {
            let (conf, value) = line
                .split_once("=")
                .expect("invalid formatting in book.toml");
            println!(" == {section:?} {line:?}");

            match section {
                "book" => match conf {
                    "title" => title = strip!(value, '\"').to_string(),
                    _ => println!(" == patten {conf} not suported"),
                },
                _ => println!("unsuported section {section}"),
            }
        }
    }

    return ();
}

async fn define_procedence(input: &str) -> PathType {
    let (prefix, _) = input.split_once('/').unwrap();
    match prefix {
        "https:" => PathType::HTTPS(input),
        _ => PathType::SYSTEM(input),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormatType {
    Bold,
    Italic,
    BoldItalic,
}
enum PathType<'a> {
    HTTPS(&'a str),
    SYSTEM(&'a str),
}

/// defines to what the md book will be parsed into
#[derive(Debug, Clone)]
pub enum ParseType {
    /// latex, .tex file
    Latex,
    /// written directily into a pdf, default mode
    Raw,
}

impl From<String> for ParseType {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_ref() {
            "latex" | "tex" => Self::Latex,
            _ => Self::Raw,
        }
    }
}

use crate::errors::Error;
use crate::errors::Result;
use regex::Regex;

#[derive(Debug)]
pub struct TagSet {
    pub h1: Regex,
    pub h2: Regex,
    pub h3: Regex,
    pub h4: Regex,
    pub h5: Regex,
    pub h6: Regex,
    pub text: Regex,
    pub include: Regex,
    pub comment: Regex,
    pub line_break: Regex,
    pub blockquote: Regex,

    // text
    pub bold: Regex,
    pub italic: Regex,
    pub bold_and_italic: Regex,
}

impl TagSet {
    pub fn md() -> Result<Self> {
        Ok(Self {
            h1: Regex::new(r"^#")?,
            h2: Regex::new(r"^##")?,
            h3: Regex::new(r"^#{3}")?,
            h4: Regex::new(r"^#{4}")?,
            h5: Regex::new(r"^#{5}")?,
            h6: Regex::new(r"^#{6}")?,
            text: Regex::new("[/c, /d]+")?,
            include: Regex::new(r"\{\{#.*}}")?,
            comment: Regex::new(r"^<!--")?,
            line_break: Regex::new(r"^$")?,
            blockquote: Regex::new(r"^>")?,
            italic: Regex::new(r"[*_].+?[*_]")?,
            bold: Regex::new(r"[*_]{2}.+?[*_]{2}")?,
            bold_and_italic: Regex::new(r"[*_]{3}.+?[*_]{3}")?,
        })
    }
    pub fn html() -> Self {
        todo!()
    }
    pub fn tex() -> Self {
        todo!()
    }
    pub fn raw() -> Self {
        todo!()
    }
}

impl std::default::Default for TagSet {
    fn default() -> Self {
        Self::md().unwrap()
    }
}

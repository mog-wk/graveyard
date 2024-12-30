use crate::tags::TagSet;
use crate::TextFormatBuilder;
use std::fs::read_to_string;
use std::path::Path;

/// parses and inserts markdown text into a pdf
pub fn parse(doc: &mut genpdf::Document, contents: &str, src: &Path, tagset: &TagSet) {
    let mut c: u8 = 0;
    for line in contents.lines() {
        // skip comments
        if tagset.comment.is_match(line) {
            continue;
        }
        dbg!(line);

        // line break
        if tagset.line_break.is_match(line) {
            doc.push(genpdf::elements::Break::new(1));
            continue;
        }

        // include statement
        if tagset.include.is_match(line) {
            let (_, path) = line.split_once(' ').expect("format error");
            let path = read_to_string(src.join(&path[..path.len() - 2]))
                .expect("invalid #include in markdown file");
            parse(doc, &path, src, tagset);
            continue;
        }

        // headers
        if tagset.h1.is_match(line) {
            let line = "= ".to_string() + &line[1..line.len()];
            doc.push(genpdf::elements::Text::new(line));
            doc.push(genpdf::elements::Break::new(1));
            continue;
        }

        // default/text processing

        let text_format_arr = TextFormatBuilder::new()
            .add(
                &crate::utils::FormatType::BoldItalic,
                tagset
                    .bold_and_italic
                    .find_iter(line)
                    .map(|m| m.range())
                    .collect(),
            )
            .add(
                &crate::utils::FormatType::Bold,
                tagset.bold.find_iter(line).map(|m| m.range()).collect(),
            )
            .add(
                &crate::utils::FormatType::Italic,
                tagset.italic.find_iter(line).map(|m| m.range()).collect(),
            )
            .build();

        doc.push(genpdf::elements::Paragraph::new(line));
        if c >= 4 {
            break;
        } else {
            c += 1;
        }
    }
    todo!();
}

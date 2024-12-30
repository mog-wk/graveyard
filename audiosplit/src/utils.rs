use crate::{
    errors::{CliError, Error},
    SPLIT_DELIMITER,
};

pub fn parse_timestamp_file(ts_path: &str) -> Result<(Vec<&str>, Vec<&str>), Error> {
    Ok(ts_path
        .lines()
        .enumerate()
        .map(|(num, line)| {
            line.split_once(SPLIT_DELIMITER)
                .ok_or(CliError::TimestampFormatError {
                    num,
                    line: line.to_string(),
                })
                .unwrap()
        })
        .collect::<(Vec<&str>, Vec<&str>)>())
}

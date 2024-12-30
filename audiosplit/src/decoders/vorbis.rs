use std::f64::{self, consts::PI};

use super::CodecFormat;

pub fn validate_checksum<'a>(checksum: &'a [u8]) -> Result<(), std::io::Error> {
    let input_len = checksum.len();
    if input_len != 4 {
        panic!("Checksum error")
    }

    Ok(())
}

pub fn mod_discrete_cos_transform(ls: &Vec<u8>) -> Vec<f64> {
    let l = ls.len();
    let N = (2 * l - 1) as f64;
    ls.iter()
        .enumerate()
        .map(|(i, n)| f64::cos((PI / N) * (*n as f64 + 0.5 + N / 2.0) * (i as f64 + 0.5)))
        .collect()
}

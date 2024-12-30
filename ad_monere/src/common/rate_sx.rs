use create::error::Result;
use std::process::Command;

pub fn get_btc() -> Result<()> {
    Command::new("curl").args([]).output()?.stdout
}

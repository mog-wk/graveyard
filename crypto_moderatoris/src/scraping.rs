use reqwest::StatusCode;

use crate::error::Result;

#[allow(unused)]
trait Scrapper {
    fn get() -> () {
        todo!()
    }
}

// coincap.io
pub(crate) fn get(url: &str) -> Result<String> {
    let body = reqwest::blocking::get(url)?;
    if body.status() != StatusCode::OK {
        tracing::error!("{}", body.status());
        return Err(crate::error::Error::RequestStatusCodeError(
            body.status().to_string(),
        ));
    }

    let body = body.text()?;

    Ok(body)
}

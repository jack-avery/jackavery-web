use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebsiteError {
    #[error("bad region: {0}")]
    BadRegion(String)
}
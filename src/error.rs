use failure::Fail;
use std::io;

#[derive(Debug, Fail)]
pub enum Error {
    /// A HTTP or connection error occurred
    #[fail(display = "HTTP client error: {}", _0)]
    ReqwestError(#[fail(cause)] reqwest::Error),

    #[fail(display = "I/O error: {}", _0)]
    IoError(#[fail(cause)] io::Error),
}

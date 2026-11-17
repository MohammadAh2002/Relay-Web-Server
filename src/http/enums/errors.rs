use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpParsingErrors {
    #[error("Invalid Request")]
    InvalidRequest,

    #[error("Invalid Request Method")]
    InvalidRequestMethod,

    #[error("Invalid Header")]
    InvalidHeader,

    #[error("Invalid Request Line")]
    InvalidRequestLine,

    #[error("Invalid HTTP request body")]
    InvalidRequestBody,

    #[error("Failed To Parse")]
    FailToParse,

    #[error("Failed to read HTTP request")]
    Io(#[from] io::Error),
}

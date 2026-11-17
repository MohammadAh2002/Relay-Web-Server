use crate::http::enums::errors::HttpParsingErrors;

#[derive(Debug, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    PATCH
}

use std::str::FromStr;

impl FromStr for HttpMethod {

    type Err = HttpParsingErrors;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "HEAD" => Ok(HttpMethod::HEAD),
            "PATCH" => Ok(HttpMethod::PATCH),
            _ => Err(HttpParsingErrors::InvalidRequestMethod),
        }
    }
}
use crate::http::models::http_header::HttpHeader;
use std::collections::HashSet;
use crate::http::enums::methods::HttpMethod;
use crate::http::models::parsed_request_body::ParsedRequestBody;

pub struct HttpRequest {
    
    pub method: HttpMethod,
    pub uri: String,
    pub version: String,
    pub headers: HashSet<HttpHeader>,
    pub body: Option<ParsedRequestBody>

}

impl HttpRequest {
    
    pub fn new( method: HttpMethod,
                uri: String,
                version: String,
                headers: HashSet<HttpHeader>,
                body: Option<ParsedRequestBody>) -> Self {
        Self {
            method: method,
            uri: uri,
            version: version,
            headers: headers,
            body: body,
        }
    }

}
use crate::http::models::http_header::HttpHeader;

pub struct ParsedRequestBody {
    pub body: Option<Vec<u8>>,
    pub trailers: Option<Vec<HttpHeader>>,
}
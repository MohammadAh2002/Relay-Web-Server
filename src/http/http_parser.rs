use std::{
    net::TcpStream,
    io::{BufReader, prelude::*},
};

use std::collections::HashSet;
use anyhow::{Context, Result};
use anyhow::bail;

use crate::http::models::{http_request::HttpRequest};
use crate::http::models::http_header::HttpHeader;
use crate::http::enums::errors::HttpParsingErrors;
use crate::http::enums::methods::HttpMethod;
use crate::http::models::parsed_request_body::ParsedRequestBody;

    pub fn parse_request(stream: TcpStream) -> Result<HttpRequest>{

        let mut buf_reader: BufReader<&TcpStream> = BufReader::new(&stream);
 
        let (method, path, version) = parse_request_line(&mut buf_reader).context(HttpParsingErrors::InvalidRequestLine).context("Failed to parse the request line")?;
        
        let headers_result: HashSet<HttpHeader> = parse_request_headers(&mut buf_reader).context(HttpParsingErrors::InvalidHeader).context("Failed to parse the request headers")?;

        let content_length = headers_result.iter()
            .find(|h| h.key.eq("Content-Length"));

        let transfer_encoding = headers_result.iter()
            .find(|h| h.key.eq("Transfer-Encoding"));

        if content_length.is_some() && transfer_encoding.is_some() {
            bail!("Request contains both Content-Length and Transfer-Encoding, only one allowed in the Request");
        }

        let body_size_header = transfer_encoding
            .or(content_length)
            .context("Missing Content-Length or Transfer-Encoding header")?;

        let body_result: Option<ParsedRequestBody> = parse_request_body(&mut buf_reader, body_size_header).context("Failed to parse body")?;
        
        let result: HttpRequest = HttpRequest::new(method, path, version,
                                                  headers_result,
                                                  body_result);
        
        Ok(result)

    }

    pub fn parse_request_line(buf_reader: &mut BufReader<&TcpStream>) -> Result<(HttpMethod, String, String)>{
    
        let mut request_line: String = String::new();

        buf_reader.read_line(&mut request_line).map_err(HttpParsingErrors::from)
                                                    .context("Failed to read HTTP request line")?;

        let mut parts = request_line.split_whitespace();

        let method = parts.next().context("Missing HTTP method")?;

        let method: HttpMethod = method
                                .parse()
                                .context("Failed to parse HTTP request method")?;

        let path = parts.next().context("Missing URI")?;

        let version = parts.next()
            .and_then(|v| v.split('/').nth(1))
            .context("Missing HTTP version")?;

        Ok((
            method,
            path.to_string(),
            version.to_string(),
        ))
        
    }

    pub fn parse_request_headers(buf_reader: &mut BufReader<&TcpStream>)-> Result<HashSet<HttpHeader>>{

        let mut headers: HashSet<HttpHeader> = HashSet::new();
        let mut line = String::new();

        loop{

            line.clear();

            buf_reader.read_line(&mut line).map_err(HttpParsingErrors::from)
                                                .context("Failed to read HTTP header")?;

            if line.is_empty() {
                 bail!("Unexpected EOF while reading HTTP headers"); // Connection closed
            }

            // Empty line means end of headers
            if line == "\r\n" {
                break;
            }

            let header: Result<HttpHeader> = process_single_header_field(&line)
                                                             .context(HttpParsingErrors::InvalidHeader)
                                                             .context("Failed to parse request headers");

            headers.insert(header?);

        }

        Ok(headers)

    }

    pub fn parse_request_body(buf_reader: &mut BufReader<&TcpStream>, body_size_header: &HttpHeader)-> Result<Option<ParsedRequestBody>>{

        if body_size_header.key.eq("Content-Length") {
            
            return proccess_content_length(buf_reader, body_size_header).context("Problme in parsing Content-Length body");
        
        }
        else if body_size_header.key.eq("Transfer-Encoding"){
            return proccess_teransfer_encoding(buf_reader, body_size_header).context("Problme in parsing Transfer-Encoding(chunked) body");
        } 
        
        return Ok(None);
        
    }

    fn process_single_header_field(header: &str)-> Result<HttpHeader>{
        
         let (key, value) = header
                            .split_once(':')
                            .context("Failed to split HTTP header: missing ':'")?;

        let key = key.trim().to_string();
        let value = value.trim().to_string();

        Ok(HttpHeader { key, value })        

    }

    fn proccess_content_length(buf_reader: &mut BufReader<&TcpStream>, body_size_header: &HttpHeader) -> Result<Option<ParsedRequestBody>>{

        let length: usize = body_size_header.value
                            .parse()
                            .context("Invalid Content-Length value")?;

        if length == 0 {
            return Ok(Some(ParsedRequestBody {
                body: None,
                trailers: None,
            }));
        }

        let mut body = vec![0u8; length];
            
        buf_reader.read_exact(&mut body).map_err(HttpParsingErrors::from)
                                                 .context("Failed to read HTTP Body")?;     

        return Ok(Some(ParsedRequestBody {
                body: Some(body),
                trailers: None,
        }));
    }

    fn proccess_teransfer_encoding(buf_reader: &mut BufReader<&TcpStream>, body_size_header: &HttpHeader) -> Result<Option<ParsedRequestBody>>{

        if body_size_header.value != "chunked"{
            bail!("Invalid Transfer-Encoding header value");
        }

        let mut body: Vec<u8> = Vec::new();

        loop{

            let mut line = String::new();
                
            buf_reader.read_line(&mut line).map_err(HttpParsingErrors::from)
                                                    .context("Failed to read HTTP body")?;

            let chunk_size_line = line.trim();

            let (size, _extensions) = chunk_size_line
                .split_once(';')
                .unwrap_or((chunk_size_line, ""));

            let chunk_size = usize::from_str_radix(size, 16)
                .context("Failed to parse HTTP chunk size from the body")?;

            if chunk_size == 0 {
                // read trailer field
                            
                let trailers: Option<Vec<HttpHeader>> = proccess_trailer(buf_reader).context("problem in parsing body trailer")?;

                return Ok(Some(ParsedRequestBody {
                    body: Some(body),
                    trailers,
                }));

            }

            let mut chunk_data = vec![0u8; chunk_size];
            buf_reader.read_exact(&mut chunk_data).map_err(HttpParsingErrors::from)
                                                 .context("Failed to read HTTP Body")?;

            body.extend_from_slice(&chunk_data);

            // Consume the CRLF after each chunk's data
            let mut crlf = [0u8; 2];

            buf_reader.read_exact(&mut crlf).map_err(HttpParsingErrors::from)
                                                 .context("Failed to read HTTP Body")?;

            if crlf != *b"\r\n" {
                bail!("Invalid chunk terminator");
            }
            
        }
    }

    fn proccess_trailer(buf_reader: &mut BufReader<&TcpStream>) -> Result<Option<Vec<HttpHeader>>>{

        let mut trailer_line = String::new();
        let mut trailers: Vec<HttpHeader> = Vec::new();

        loop {
            
            trailer_line.clear();

            buf_reader
                .read_line(&mut trailer_line)
                .context("Failed to read HTTP trailer")?;

            // Unexpected end of stream
            if trailer_line.is_empty() {
                bail!("Unexpected EOF while reading HTTP trailers");
            }
            
            // End of trailer section
            if trailer_line == "\r\n" {
                break;
            }

            let trailer = process_single_header_field(&trailer_line)?;
                        
            trailers.push(trailer);

        }
            

        if trailers.is_empty() {
            Ok(None)
        } else {
            Ok(Some(trailers))
        }

    }
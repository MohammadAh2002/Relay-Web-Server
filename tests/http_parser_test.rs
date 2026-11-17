use relay_web_server::http::http_parser;
use relay_web_server::{http::enums::methods::HttpMethod, http::models::http_header::HttpHeader};
use std::io::Write;
use std::thread;
use std::{
    io::BufReader,
    net::{TcpListener, TcpStream},
};

#[cfg(test)]
mod tests_main_parse_function {
    use super::*;

    #[test]
    fn test_success_request() {
        let request = "GET /users HTTP/1.1\r\n\
            Host: localhost\r\n\
            Accept: application/json\r\n\
            Content-Length: 0\r\n\
            \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_ok());
    }

    #[test]
    fn test_fail_request() {
        let request = "";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_err());
    }

    #[test]
    fn test_request_with_both_missing_content_length_and_transfer_encoding_headers() {
        let request = "GET /users HTTP/1.1\r\n\
            Host: localhost\r\n\
            Accept: application/json\r\n\
            \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_err());
    }

    #[test]
    fn test_request_with_both_content_length_and_transfer_encoding_headers() {
        let request = "GET /users HTTP/1.1\r\n\
            Host: localhost\r\n\
            Accept: application/json\r\n\
            Content-Length: 10\r\n\
            Transfer-Encoding: chunked\r\n\
            \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_err());
    }

    #[test]
    fn test_request_with_case_sensetive_content_length_and_transfer_encoding_headers() {
        let request = "GET /users HTTP/1.1\r\n\
            Host: localhost\r\n\
            Accept: application/json\r\n\
            content-length: 10\r\n\
            \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_err());

        let request = "GET /users HTTP/1.1\r\n\
            Host: localhost\r\n\
            Accept: application/json\r\n\
            transfer-encoding: chunked\r\n\
            \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_err());
    }
}

#[cfg(test)]
mod tests_parse_request_line_function {
    use super::*;

    #[test]
    fn test_parse_no_request_line() {
        let request = "   Host: localhost\r\n\
            Accept: application/json\r\n\
            Content-Length: 0\r\n\
            \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_success_get_request_line() {
        let request = "GET /users HTTP/1.1\r\n\
            Host: localhost\r\n\
            Accept: application/json\r\n\
            Content-Length: 0\r\n\
            \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(result.method, HttpMethod::GET);
    }
    #[test]
    fn test_parse_success_post_request_line() {
        let request = "POST /users HTTP/1.1\r\n\
            Host: localhost\r\n\
            Accept: application/json\r\n\
            Content-Length: 0\r\n\
            \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(result.method, HttpMethod::POST);
    }

    #[test]
    fn test_parse_wrong_exting_method_request() {
        let request = "test /users HTTP/1.1\r\n\
            Host: localhost\r\n\
            Accept: application/json\r\n\
            \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_none_exting_method_request() {
        let request = "/users HTTP/1.1\r\n\
            Host: localhost\r\n\
            Accept: application/json\r\n\
            \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_no_uri_method_request() {
        let request = "GET HTTP/1.1\r\n\
            Host: localhost\r\n\
            Accept: application/json\r\n\
            \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_no_http_version_method_request() {
        let request = "GET /users\r\n\
            Host: localhost\r\n\
            Accept: application/json\r\n\
            \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_err());
    }
}

#[cfg(test)]
mod tests_parse_request_headers_function {
    use super::*;

    #[test]
    fn test_parse_success_header() {
        let request = "GET /users HTTP/1.1\r\n\
            Host: localhost\r\n\
            Accept: application/json\r\n\
            Content-Length: 0\r\n\
            \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_one_wrong_header() {
        let request = "GET /users\r\n\
         Host:\r\n\
        Accept: application/json\r\n\
        Content-Length: 10\r\n\
        \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_eof_header() {
        let request = "GET /users\r\n\
        Host:\r\n\
        \r\n
        Accept: application/json\r\n\
        Content-Length: 10\r\n\
        \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_no_value_header() {
        let request = "GET /users\r\n\
         Host:\r\n\
        Accept: application/json\r\n\
        Content-Length: 10\r\n\
        \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_no_key_header() {
        let request = "GET /users\r\n\
        : localhost\r\n\
        Accept: application/json\r\n\
        Content-Length: 10\r\n\
        \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_no_new_lines_between_headers() {
        let request = "GET /users\r\n\
        Host:\r\n\
        Accept: application/json\
        Content-Length: 10\r\n\
        \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_colon_header() {
        let request = "GET /users\r\n\
         Host localhost\r\n\
        Accept: application/json\r\n\
        Content-Length: 10\r\n\
        \r\n";

        // 2. Create a TCP stream containing the request
        let stream = mock_tcp_stream(request);

        let result = http_parser::parse_request(stream);

        assert!(result.is_err());
    }
}

#[cfg(test)]
mod tests_parse_body_function {

    use super::*;

    fn content_length(length: &str) -> HttpHeader {
        HttpHeader {
            key: "Content-Length".to_string(),
            value: length.to_string(),
        }
    }
    fn transfer_encoding(value: &str) -> HttpHeader {
        HttpHeader {
            key: "Transfer-Encoding".to_string(),
            value: value.to_string(),
        }
    }

    #[test]
    fn test_success_request_with_body_content_length_header() {
        let request_body = "{\"name\":\"mohammad\",\"age\":24,\"city\":\"Jerusalem\"}";

        let stream = mock_tcp_stream(request_body);

        let mut buf_reader = BufReader::new(&stream);

        let result = http_parser::parse_request_body(
            &mut buf_reader,
            &content_length(&request_body.len().to_string()),
        );

        assert!(result.is_ok());

        let result = result.unwrap().unwrap();

        assert!(result.body.is_some());
        assert!(result.trailers.is_none());
        assert_eq!(result.body.as_ref().unwrap().len(), request_body.len());
    }

    #[test]
    fn test_invalid_content_length() {
        let request_body = "{\"name\":\"mohammad\",\"age\":24,\"city\":\"Jerusalem\"}";

        let stream = mock_tcp_stream(request_body);

        let mut buf_reader = BufReader::new(&stream);

        let result =
            http_parser::parse_request_body(&mut buf_reader, &content_length(&"abs".to_string()));

        assert!(result.is_err());

        let stream = mock_tcp_stream(request_body);

        let mut buf_reader = BufReader::new(&stream);

        let result =
            http_parser::parse_request_body(&mut buf_reader, &content_length(&"-1".to_string()));

        assert!(result.is_err());

        let stream = mock_tcp_stream(request_body);

        let mut buf_reader = BufReader::new(&stream);

        let result =
            http_parser::parse_request_body(&mut buf_reader, &content_length(&"".to_string()));

        assert!(result.is_err());
    }

    #[test]
    fn test_success_request_with_content_length_bigger_than_available_data() {
        let request_body = "{\"name\":\"mohammad\",\"age\":24,\"city\":\"Jerusalem\"}";

        let stream = mock_tcp_stream(request_body);

        let mut buf_reader = BufReader::new(&stream);

        let result = http_parser::parse_request_body(
            &mut buf_reader,
            &content_length(&(request_body.len() + 1).to_string()),
        );

        assert!(result.is_err());
    }
    #[test]
    fn test_success_request_with_content_length_smaller_than_available_data() {
        let request_body = "{\"name\":\"mohammad\",\"age\":24,\"city\":\"Jerusalem\"}";

        let stream = mock_tcp_stream(request_body);

        let mut buf_reader = BufReader::new(&stream);

        let result =
            http_parser::parse_request_body(&mut buf_reader, &content_length(&9.to_string()));

        assert!(result.is_ok());

        let result = result.unwrap().unwrap();

        assert!(result.body.is_some());
        assert_eq!(result.body.as_ref().unwrap().len(), 9);
    }

    #[test]
    fn test_success_request_with_body_transfer_encoding_header() {
        let request_body = "13\r\n{\"name\":\"mohammad\",\r\n\
            1C\r\n\"age\":24,\"city\":\"Jerusalem\"}\r\n\
            0\r\n\r\n";

        let stream = mock_tcp_stream(request_body);
        let mut buf_reader = BufReader::new(&stream);

        let result =
            http_parser::parse_request_body(&mut buf_reader, &transfer_encoding("chunked"));

        assert!(result.is_ok());

        let result = result.unwrap().unwrap();

        let expected_body = "{\"name\":\"mohammad\",\"age\":24,\"city\":\"Jerusalem\"}";

        // Check the decoded body
        assert_eq!(result.body.as_ref().unwrap(), expected_body.as_bytes());

        assert!(result.trailers.is_none());
    }

    #[test]
    fn test_fail_request_with_incorrect_chunk_terminator() {
        let request_body = "13\r\n{\"name\":\"mohammad\",\r\n\
            1C\r\n\"age\":24,\"city\":\"Jerusalem\"}\r\n\
            0\r\n\r";

        let stream = mock_tcp_stream(request_body);
        let mut buf_reader = BufReader::new(&stream);

        let result =
            http_parser::parse_request_body(&mut buf_reader, &transfer_encoding("chunked"));

        assert!(result.is_err());
    }

    #[test]
    fn test_fail_request_with_invalid_chunk_size_transfer_encoding_header() {
        let request_body = "xfh\r\n{\"name\":\"mohammad\",\r\n\
            1C\r\n\"age\":24,\"city\":\"Jerusalem\"}\r\n\
            0\r\n\r\n";

        let stream = mock_tcp_stream(request_body);
        let mut buf_reader = BufReader::new(&stream);

        let result =
            http_parser::parse_request_body(&mut buf_reader, &transfer_encoding("chunked"));

        assert!(result.is_err());
    }

    #[test]
    fn test_request_worng_body_size_header() {
        let request_body1 = "13\r\n{\"name\":\"mohammad\",\r\n\
            1C\r\n\"age\":24,\"city\":\"Jerusalem\"}\r\n\
            0\r\n\r\n";

        let stream1 = mock_tcp_stream(request_body1);

        let mut buf_reader1 = BufReader::new(&stream1);

        let result1 = http_parser::parse_request_body(
            &mut buf_reader1,
            &HttpHeader {
                key: "wrong_header".to_string(),
                value: "chunked".to_string(),
            },
        );

        assert!(result1.unwrap().is_none());

        let request_body2 = "{\"name\":\"mohammad\",\"age\":24,\"city\":\"Jerusalem\"}";

        let stream2 = mock_tcp_stream(request_body2);

        let mut buf_reader2 = BufReader::new(&stream2);

        let result2 = http_parser::parse_request_body(
            &mut buf_reader2,
            &HttpHeader {
                key: "wrong_header".to_string(),
                value: "500".to_string(),
            },
        );

        assert!(result2.unwrap().is_none());
    }

    #[test]
    fn test_success_request_with_trailers() {
        let request_body = "4\r\nWiki\r\n\
            0\r\n\
            Checksum: 12345\r\n\
            \r\n";

        let stream = mock_tcp_stream(request_body);
        let mut buf_reader = BufReader::new(&stream);

        let result =
            http_parser::parse_request_body(&mut buf_reader, &transfer_encoding("chunked"));

        assert!(result.is_ok());

        let result = result.unwrap().unwrap();

        assert_eq!(result.body.as_ref().unwrap(), b"Wiki");

        assert!(result.trailers.is_some());

        assert_eq!(result.trailers.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_success_request_with_multiple_trailers() {
        let request_body = "4\r\nWiki\r\n\
            0\r\n\
            Checksum: 12345\r\n\
            size: 55mb\r\n\
            sender: mohammad\r\n\
            \r\n";

        let stream = mock_tcp_stream(request_body);
        let mut buf_reader = BufReader::new(&stream);

        let result =
            http_parser::parse_request_body(&mut buf_reader, &transfer_encoding("chunked"));

        assert!(result.is_ok());

        let result = result.unwrap().unwrap();

        assert_eq!(result.body.as_ref().unwrap(), b"Wiki");

        assert!(result.trailers.is_some());

        assert_eq!(result.trailers.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn test_wrong_transfer_encoding_header_value() {
        let request_body = "13\r\n{\"name\":\"mohammad\",\r\n\
            1C\r\n\"age\":24,\"city\":\"Jerusalem\"}\r\n\
            0\r\n\r\n";

        let stream = mock_tcp_stream(request_body);
        let mut buf_reader = BufReader::new(&stream);

        let result = http_parser::parse_request_body(&mut buf_reader, &transfer_encoding("wrong"));

        assert!(result.is_err());
    }

    #[test]
    fn test_fail_request_with_bad_trailers() {
        let request_body = "4\r\nWiki\r\n\
            0\r\n\
            Checksum 12345\r\n\
            \r\n";

        let stream = mock_tcp_stream(request_body);
        let mut buf_reader = BufReader::new(&stream);

        let result =
            http_parser::parse_request_body(&mut buf_reader, &transfer_encoding("chunked"));

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_success_with_empty_trailers() {
        let request_body = "4\r\nWiki\r\n\
            0\r\n\
            \r\n";

        let stream = mock_tcp_stream(request_body);
        let mut buf_reader = BufReader::new(&stream);

        let result =
            http_parser::parse_request_body(&mut buf_reader, &transfer_encoding("chunked"));

        assert!(result.is_ok());

        let parsed = result.unwrap().unwrap();
        assert!(parsed.trailers.is_none());
    }

    #[test]
    fn test_fail_request_with_missing_trailer_ending() {
        let request_body = "4\r\nWiki\r\n\
                            0\r\n";

        let stream = mock_tcp_stream(request_body);
        let mut buf_reader = BufReader::new(&stream);

        let result =
            http_parser::parse_request_body(&mut buf_reader, &transfer_encoding("chunked"));

        assert!(result.is_err());
    }

    #[test]
    fn test_fail_request_with_no_end_trailers() {
        let request_body = "4\r\nWiki\r\n\
            0\r\n\
            Checksum: 12345\r\n\
            ";

        let stream = mock_tcp_stream(request_body);
        let mut buf_reader = BufReader::new(&stream);

        let result =
            http_parser::parse_request_body(&mut buf_reader, &transfer_encoding("chunked"));

        assert!(result.is_err());
    }

    #[test]
    fn test_request_with_case_sensetive_content_length_and_transfer_encoding_headers() {
        let request_body = "{\"name\":\"mohammad\",\"age\":24,\"city\":\"Jerusalem\"}";

        let stream = mock_tcp_stream(request_body);
        let mut buf_reader = BufReader::new(&stream);

        let result = http_parser::parse_request_body(
            &mut buf_reader,
            &HttpHeader {
                key: "content-length".to_string(),
                value: request_body.len().to_string(),
            },
        );

        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert!(parsed.is_none());

        let request_body = "13\r\n{\"name\":\"mohammad\",\r\n\
            1C\r\n\"age\":24,\"city\":\"Jerusalem\"}\r\n\
            0\r\n\r\n";

        let stream = mock_tcp_stream(request_body);
        let mut buf_reader = BufReader::new(&stream);

        let result = http_parser::parse_request_body(
            &mut buf_reader,
            &HttpHeader {
                key: "transfer-encoding".to_string(),
                value: "chunked".to_string(),
            },
        );

        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert!(parsed.is_none());
    }

    #[test]
    fn test_fail_incomplete_chunk() {
        let request_body = "A\r\nWiki\r\n";

        let stream = mock_tcp_stream(request_body);
        let mut buf_reader = BufReader::new(&stream);

        let result =
            http_parser::parse_request_body(&mut buf_reader, &transfer_encoding("chunked"));

        assert!(result.is_err());
    }

    #[test]
    fn test_fail_size_larger_than_available_chunk_data() {
        let request_body = "A\r\nmohammad\r\n";

        let stream = mock_tcp_stream(request_body);
        let mut buf_reader = BufReader::new(&stream);

        let result =
            http_parser::parse_request_body(&mut buf_reader, &transfer_encoding("chunked"));

        assert!(result.is_err());
    }

    #[test]
    fn test_fail_size_smaller_than_available_chunk_data() {
        let request_body = "5\r\nmohammad\r\n";

        let stream = mock_tcp_stream(request_body);
        let mut buf_reader = BufReader::new(&stream);

        let result =
            http_parser::parse_request_body(&mut buf_reader, &transfer_encoding("chunked"));

        assert!(result.is_err());
    }
}

fn mock_tcp_stream(request: &str) -> TcpStream {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();

    let request = request.to_string();

    thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();

        stream.write_all(request.as_bytes()).unwrap();
    });

    TcpStream::connect(address).unwrap()
}

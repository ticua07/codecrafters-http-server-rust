use std::collections::HashMap;

#[derive(Debug)]
pub struct HTTPRequest {
    pub method: HTTPMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
}

#[derive(Debug)]
pub enum HTTPMethod {
    GET,
    POST,
    INVALID,
}

pub const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n";

impl HTTPRequest {
    pub fn new(method: HTTPMethod, path: String, headers: HashMap<String, String>) -> Self {
        Self {
            method,
            path,
            headers,
        }
    }
}

pub fn parse_request(request_string: &str) -> HTTPRequest {
    let lines: Vec<String> = request_string.lines().map(String::from).collect();
    let mut first_line = lines[0].split_ascii_whitespace();

    let method = match first_line.next().expect("Couldn't parse request") {
        "GET" => HTTPMethod::GET,
        "POST" => HTTPMethod::POST,
        _ => HTTPMethod::INVALID,
    };

    let route = first_line.next().expect("Couldn't parse request");

    println!("{} -> {}", method, route);

    let mut headers: HashMap<String, String> = HashMap::new();

    for header in lines.iter().skip(1) {
        if header.is_empty() {
            break;
        }
        let header_values: Vec<String> = header.split(":").map(String::from).collect();
        headers.insert(header_values[0].clone(), header_values[1].trim().to_owned());
    }

    HTTPRequest::new(method, route.to_string(), headers)
}

pub fn create_response(code: String, content_type: String, body: String) -> String {
    let cont_len = body.len();

    return format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        code, content_type, cont_len, body
    );
}

impl std::fmt::Display for HTTPMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HTTPMethod::GET => write!(f, "GET"),
            HTTPMethod::POST => write!(f, "POST"),
            HTTPMethod::INVALID => write!(f, "INVALID"),
        }
    }
}

use std::fs::File;
use std::io::prelude::*;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct HTTPRequest {
    pub method: HTTPMethod,
    pub path: String,
    pub body: String,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum HTTPMethod {
    GET,
    POST,
    INVALID,
}

pub const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n";

impl HTTPRequest {
    pub fn new(
        method: HTTPMethod,
        path: String,
        body: String,
        headers: HashMap<String, String>,
    ) -> Self {
        Self {
            method,
            path,
            body,
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

    let body = lines
        //* Here I check for empty and not \r\n (separator between headers and body)
        //* due to the \r\n getting removed by split_ascii_whitespace
        .split_at(lines.iter().position(|r| r.is_empty()).unwrap() + 1)
        .1 // .0 just contains headers
        .first()
        .unwrap()
        .to_owned();

    println!("[body] {}", body);

    HTTPRequest::new(method, route.to_string(), body, headers)
}

pub fn create_response(code: String, content_type: String, body: String) -> String {
    let cont_len = body.len();

    println!("[SENDING RESPONSE]: {} {} -> {}", code, content_type, body);

    return format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        code, content_type, cont_len, body
    );
}

pub fn serve_file(path: PathBuf) -> String {
    if path.is_file() {
        return format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
            std::fs::metadata(&path).unwrap().len(),
            std::fs::read_to_string(&path).unwrap()
        );
    } else {
        String::from(NOT_FOUND_RESPONSE)
    }
}
pub fn save_file(path: PathBuf, content: String) -> String {
    if !path.exists() {
        let content = content.replace("\0", ""); // Remove all '\0's
        let content = content.replace(r"\r\n", "\n"); // Interpret '\r\n' as newlines
        let content = content.replace(r"\n", "\n"); // Interpret '\n' as newlines

        let mut file = File::create(path).expect("Couldn't create file");
        file.write_all(content.as_bytes())
            .expect("Couldn't write to file");
        create_response(
            "201 CREATED".to_string(),
            "text/plain".to_string(),
            "".to_string(),
        )
    } else {
        create_response(
            "500 INTERNAL SERVER ERROR".to_string(),
            "text/plain".to_string(),
            "".to_string(),
        )
    }
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

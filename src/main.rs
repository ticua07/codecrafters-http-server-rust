// Uncomment this block to pass the first stage
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use utils::{HTTPMethod, NOT_FOUND_RESPONSE, OK_RESPONSE};

use crate::utils::HTTPRequest;
mod utils;

fn handle_conn(stream: &mut TcpStream) {
    let mut request_buffer = [0; 512];

    stream.read(&mut request_buffer).unwrap();

    let request_str = std::str::from_utf8(&request_buffer).unwrap();
    let req = parse_request(request_str);

    let response = match req.path.as_str() {
        "/" => String::from(OK_RESPONSE),
        _ => String::from(NOT_FOUND_RESPONSE),
    };

    stream
        .write(response.as_bytes())
        .expect("Couldn't return response");
}

fn parse_request(request_string: &str) -> HTTPRequest {
    let lines: Vec<&str> = request_string.lines().collect();
    let mut first_line = lines[0].split_ascii_whitespace();

    let method = match first_line.next().unwrap() {
        "GET" => HTTPMethod::GET,
        "POST" => HTTPMethod::POST,
        _ => HTTPMethod::INVALID,
    };

    let route = first_line.next().unwrap();
    println!("{} -> {}", method, route);

    HTTPRequest::new(method, route.to_string())
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_conn(&mut stream);
                stream
                    .write(b"HTTP/1.1 200 OK\r\n\r\n")
                    .expect("Failed to send msg");
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

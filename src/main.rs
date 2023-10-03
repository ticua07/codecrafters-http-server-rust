// Uncomment this block to pass the first stage
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use utils::NOT_FOUND_RESPONSE;

use crate::cli::get_directory;
use crate::utils::{create_response, parse_request};
mod cli;
mod utils;

fn handle_conn(stream: &mut TcpStream) {
    let mut request_buffer = [0; 512];

    stream.read(&mut request_buffer).unwrap();

    let request_str = std::str::from_utf8(&request_buffer).unwrap();
    println!("[DATA]: {}", request_str);
    let req = parse_request(request_str);

    let response = match req.path.as_str() {
        "/" => create_response(
            "200 OK".to_string(),
            "text/plain".to_string(),
            String::new(),
        ),
        "/user-agent" => create_response(
            "200 OK".to_string(),
            "text/plain".to_string(),
            req.headers
                .get(&"User-Agent" as &str)
                .expect("Couldn't find header User-Agent")
                .clone(),
        ),
        s if s.starts_with("/echo/") => {
            let temp: String = req.path.replace("/echo/", "");
            let response = create_response("200 OK".to_string(), "text/plain".to_string(), temp);
            response
        }
        _ => String::from(NOT_FOUND_RESPONSE),
    };

    stream
        .write(response.as_bytes())
        .expect("Couldn't return response");
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    println!("GETTING ARGUMENTS");
    println!("{:?}", get_directory());

    let result = match get_directory() {
        Some(res) => res,
        None => panic!("--directory argument is invalid"),
    };

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    handle_conn(&mut stream);
                });
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

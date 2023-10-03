use std::fs::{self, File};
// Uncomment this block to pass the first stage
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::path::{self, PathBuf};
use std::thread;
use utils::NOT_FOUND_RESPONSE;

use crate::cli::get_directory;
use crate::utils::{create_response, parse_request, serve_file};
mod cli;
mod utils;

fn handle_conn(stream: &mut TcpStream, directory: PathBuf) {
    let mut request_buffer = [0; 512];

    stream.read(&mut request_buffer).unwrap();

    let request_str = std::str::from_utf8(&request_buffer).unwrap();
    println!("[DATA]: {}", request_str);
    println!("[DIRECTORY]: {:?}", directory);

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
        s if s.starts_with("/files/") => {
            let filename = req.path.replace("/files/", "");

            for entry in directory.read_dir().expect("read_dir call failed") {
                if let Ok(entry) = entry {
                    println!("{:?}", entry.path());
                }
            }

            println!("[PATH]: {:#?}", directory);
            serve_file(directory.join(filename))
        }
        s if s.starts_with("/echo/") => {
            let temp: String = req.path.replace("/echo/", "");
            let response = create_response("200 OK".to_string(), "text/plain".to_string(), temp);
            response
        }
        _ => String::from(create_response(
            "404 NOT FOUND".to_string(),
            "text/plain".to_string(),
            "not found.".to_string(),
        )),
    };

    stream
        .write(response.as_bytes())
        .expect("Couldn't return response");
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    println!("GETTING ARGUMENTS");

    let files_dir = get_directory();
    println!("{:?}", files_dir);
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    thread::scope(|_| {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let files_dir = files_dir.clone();
                    thread::spawn(move || {
                        handle_conn(&mut stream, files_dir);
                    });
                    println!("accepted new connection");
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    });
}

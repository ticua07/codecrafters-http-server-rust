#[derive(Debug)]
pub struct HTTPRequest {
    pub method: HTTPMethod,
    pub path: String,
}

#[derive(Debug)]
pub enum HTTPMethod {
    GET,
    POST,
    INVALID,
}

pub const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
pub const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n";

impl HTTPRequest {
    pub fn new(method: HTTPMethod, path: String) -> Self {
        Self { method, path }
    }
}

pub fn create_response(code: String, content_type: String, body: String) -> String {
    let cont_len = body.len();

    println!("PLEASE");
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

use std::io::Write;

use crate::response::Response;

pub fn send_response(mut stream: std::net::TcpStream, response: Response) {
    let response = response.parse();
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

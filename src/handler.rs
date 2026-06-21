use std::io::Write;

const CRLF: &str = "\r\n";
const HTTP_VERSION: &str = "HTTP/1.1";

pub fn handle_request(mut stream: std::net::TcpStream) {
    produce_response(&mut stream);
}

fn produce_response(stream: &mut std::net::TcpStream) {
    let mut response = String::new();
    // STATUS LINE
    response.push_str(&format!("{} ", HTTP_VERSION));
    response.push_str("200 ");
    response.push_str("OK");

    response.push_str(CRLF);
    // HEADERS

    response.push_str(CRLF);

    // RESPONSE BODY

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

const CRLF: &str = "\r\n";
const HTTP_VERSION: &str = "HTTP/1.1";

pub fn handle_request(stream: std::net::TcpStream) {
    produce_response();
}

fn produce_response() {
    let mut response = String::new();
    // STATUS LINE
    response.push_str(&format!("{}\n", HTTP_VERSION));
    response.push_str("200\n");
    response.push_str("OK\n");

    response.push_str(CRLF);
    // HEADERS

    response.push_str(CRLF);

    // RESPONSE BODY

    print!("{}", &response);
}

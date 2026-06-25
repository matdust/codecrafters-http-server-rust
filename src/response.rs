const CRLF: &str = "\r\n";
const HTTP_VERSION: &str = "HTTP/1.1";

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    msg: String,
}

impl Response {
    pub fn new(status_code: StatusCode) -> Self {
        Self {
            status_code,
            msg: status_code.clone().reason_phrase(),
        }
    }

    pub fn produce(&self) -> String {
        let mut response = String::new();
        // STATUS LINE
        response.push_str(&format!("{} ", HTTP_VERSION));
        response.push_str("200 ");
        response.push_str("OK");

        response.push_str(CRLF);
        // HEADERS

        response.push_str(CRLF);

        // RESPONSE BODY
        //

        response
    }
}

#[derive(Debug, Copy, Clone)]
pub enum StatusCode {
    Ok,
    NotFound,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> String {
        match self {
            StatusCode::Ok => "OK".to_string(),
            StatusCode::NotFound => "Not Found".to_string(),
        }
    }
}

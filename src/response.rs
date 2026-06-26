const CRLF: &str = "\r\n";
const HTTP_VERSION: &str = "HTTP/1.1";

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    msg: String,
    pub body: Option<String>,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status_code: StatusCode::NotFound,
            msg: String::default(),
            body: None,
        }
    }
}

impl Response {
    pub fn new(status_code: StatusCode, msg: &str) -> Self {
        Self {
            status_code,
            msg: msg.to_string(),
            body: None,
        }
    }

    pub fn not_found() -> Self {
        Self {
            status_code: StatusCode::NotFound,
            msg: String::default(),
            body: None,
        }
    }

    pub fn parse(&self) -> String {
        let mut response = String::new();
        // STATUS LINE
        response.push_str(&format!("{} ", HTTP_VERSION));
        response.push_str(self.status_code.status_code_value());
        response.push(' ');
        response.push_str(self.status_code.reason_phrase());

        response.push_str(CRLF);
        // HEADERS

        response.push_str(CRLF);

        // RESPONSE BODY
        if self.body.is_some() {
            response.push_str(&self.body.clone().unwrap());
        }
        response
    }
}

#[derive(Debug, Copy, Clone)]
pub enum StatusCode {
    Ok,
    NotFound,
}

impl StatusCode {
    pub fn status_code_value(&self) -> &str {
        match self {
            StatusCode::Ok => "200",
            StatusCode::NotFound => "404",
        }
    }
    pub fn reason_phrase(&self) -> &str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::NotFound => "Not Found",
        }
    }
}

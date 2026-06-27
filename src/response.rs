use crate::request::HeaderName;

const CRLF: &str = "\r\n";
const HTTP_VERSION: &str = "HTTP/1.1";

pub struct ResponseBuilder {
    status_code: StatusCode,
    headers: Vec<(HeaderName, String)>,
}

impl Default for ResponseBuilder {
    fn default() -> Self {
        Self {
            status_code: StatusCode::NotFound,
            headers: Default::default(),
        }
    }
}

impl ResponseBuilder {
    pub fn status_code(&mut self, status_code: StatusCode) -> &mut Self {
        self.status_code = status_code;
        self
    }

    pub fn header(&mut self, header: HeaderName, value: &str) -> &mut Self {
        self.headers.push((header, value.to_string()));
        self
    }

    pub fn build(&mut self) -> Response {
        Response {
            status_code: self.status_code,
            headers: self.headers.clone(),
            body: None,
        }
    }
}

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    headers: Vec<(HeaderName, String)>,
    pub body: Option<String>,
}

impl Response {
    pub fn not_found() -> Self {
        Self {
            status_code: StatusCode::NotFound,
            headers: Vec::default(),
            body: None,
        }
    }

    pub fn parse(&self) -> String {
        let mut response = String::new();
        // STATUS LINE
        response.push_str(&format!("{} ", HTTP_VERSION));
        response.push_str(self.status_code.as_status_code_number());
        response.push(' ');
        response.push_str(self.status_code.reason_phrase());

        response.push_str(CRLF);
        // HEADERS
        for header in &self.headers {
            response.push_str(&format!("{}:{}{}", header.0.as_str(), header.1, CRLF));
        }

        response.push_str(CRLF);

        // RESPONSE BODY
        if self.body.is_some() {
            response.push_str(&self.body.clone().unwrap());
        }
        response
    }
}

pub enum ContentType {
    TextPlain,
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::TextPlain => "text/plain",
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum StatusCode {
    Ok,
    NotFound,
}

impl StatusCode {
    pub fn as_status_code_number(&self) -> &str {
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

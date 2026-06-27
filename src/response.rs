use crate::header::HeaderName;

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

    pub fn bad_request() -> Self {
        Self {
            status_code: StatusCode::BadRequest,
            headers: Vec::default(),
            body: None,
        }
    }

    pub fn internal_server_error() -> Self {
        Self {
            status_code: StatusCode::InternalServerError,
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

#[derive(Debug, Copy, Clone)]
pub enum StatusCode {
    // 2xx
    Ok,
    Created,
    // 4xx
    NotFound,
    BadRequest,
    // 5xx
    InternalServerError,
}

impl StatusCode {
    pub fn as_status_code_number(&self) -> &str {
        match self {
            StatusCode::Ok => "200",
            StatusCode::NotFound => "404",
            StatusCode::Created => "201",
            StatusCode::BadRequest => "400",
            StatusCode::InternalServerError => "500",
        }
    }
    pub fn reason_phrase(&self) -> &str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::NotFound => "Not Found",
            StatusCode::Created => "Created",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::InternalServerError => "Internal Server Error",
        }
    }
}

const CRLF: &str = "\r\n";
const HTTP_VERSION: &str = "HTTP/1.1";

pub struct ResponseBuilder {
    status_code: StatusCode,
    headers: Vec<ResponseHeader>,
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

    pub fn header(&mut self, header: ResponseHeader) -> &mut Self {
        self.headers.push(header);
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
    headers: Vec<ResponseHeader>,
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
        response.push_str(self.status_code.status_code_value());
        response.push(' ');
        response.push_str(self.status_code.reason_phrase());

        response.push_str(CRLF);
        // HEADERS
        for header in &self.headers {
            response.push_str(&format!("{}:{}{}", header.key, header.value, CRLF));
        }

        response.push_str(CRLF);

        // RESPONSE BODY
        if self.body.is_some() {
            response.push_str(&self.body.clone().unwrap());
        }
        response
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ResponseHeader {
    key: String,
    value: String,
}

impl ResponseHeader {
    pub fn content_type(content_type: ContentType) -> Self {
        Self {
            key: "Content-Type".to_string(),
            value: match content_type {
                ContentType::TextPlain => "text/plain".to_string(),
            },
        }
    }

    pub fn content_length(length: usize) -> Self {
        Self {
            key: "Content-Length".to_string(),
            value: length.to_string(),
        }
    }
}

pub enum ContentType {
    TextPlain,
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

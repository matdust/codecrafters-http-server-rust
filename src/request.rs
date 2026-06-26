use std::collections::HashMap;

use crate::request;

pub const CRLF: &str = "\r\n";

#[derive(Debug)]
pub struct Request {
    lines: Lines,
    headers: Headers,
    body: Body,
}

impl Request {
    pub fn parse(request: Vec<String>) -> Self {
        let lines = Lines::new(&request[0].split_whitespace().collect::<Vec<_>>());

        Self {
            lines,
            headers: Headers::new(&[]),
            body: Body::new(&[]),
        }
    }

    pub fn http_method(&self) -> HttpMethod {
        self.lines.http_method.clone()
    }

    pub fn url(&self) -> &str {
        &self.lines.url
    }
}

#[derive(Debug)]
pub struct Lines {
    http_method: HttpMethod,
    url: String,
    http_version: HttpVersion,
}

impl Lines {
    pub fn new(lines: &[&str]) -> Self {
        let http_method = HttpMethod::from_str(lines[0]);
        let request_target = lines[1];
        let http_version = HttpVersion::from_str(lines[2]);

        Self {
            http_method,
            url: request_target.to_string(),
            http_version,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum HttpMethod {
    POST,
    GET,
}

impl HttpMethod {
    pub fn from_str(content: &str) -> Self {
        match content.to_uppercase().as_str() {
            "POST" => Self::POST,
            "GET" => Self::GET,
            _ => panic!("cannot parse {} http method", content),
        }
    }
}

#[derive(Debug)]
pub enum HttpVersion {
    HTTP11,
    HTTP12,
    HTTP13,
}

impl HttpVersion {
    pub fn from_str(content: &str) -> Self {
        match content.to_uppercase().as_str() {
            "HTTP/1.1" => Self::HTTP11,
            "HTTP/1.2" => Self::HTTP12,
            "HTTP/1.3" => Self::HTTP13,
            _ => panic!("cannot parse {} http version", content),
        }
    }
}

#[derive(Debug)]
pub struct Headers {}
impl Headers {
    // TODO: implement
    fn new(header_section: &[&str]) -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct Body {}
impl Body {
    // TODO: implement
    fn new(body: &[&str]) -> Self {
        Self {}
    }
}

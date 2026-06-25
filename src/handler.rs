use std::io::Write;

use crate::{
    request::{HttpMethod, Request},
    response::{Response, StatusCode},
};

// pub struct Handler {
//     http_method: HttpMethod,
//     url: String,
// }
//
// impl Handler {
//     pub fn handle_request(&self) {}
// }

pub fn handle_request(mut stream: std::net::TcpStream) {
    produce_response(&mut stream);
}

fn produce_response(stream: &mut std::net::TcpStream) {
    let response = Response::new(StatusCode::Ok).produce();
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub trait Handler: std::fmt::Debug {
    fn handle_request(&self, req: &Request) -> Response;
}

pub struct Foo {
    http_method: HttpMethod,
    url: String,
}

impl std::fmt::Debug for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Foo handler for {:?} {}", self.http_method, self.url)
    }
}

impl Handler for Foo {
    fn handle_request(&self, req: &Request) -> Response {
        todo!()
    }
}

// impl std::fmt::Debug for dyn Handler {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

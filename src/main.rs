use std::io::{BufRead, BufReader, Read, Write};
#[allow(unused_imports)]
use std::net::TcpListener;

use crate::{handler::Handler, request::Request, response::Response, router::Router};

mod handler;
mod request;
mod response;
mod router;
mod sender;

const PORT: u16 = 4221;

fn main() {
    let mut router = Router::default();

    let _ = router.add_route(request::HttpMethod::GET, "/", &RootHandler {});

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let buf_reader = BufReader::new(&stream);

                let payload = buf_reader
                    .lines()
                    .map(|result| result.unwrap())
                    .take_while(|line| !line.is_empty())
                    .collect();

                let request = Request::parse(payload);
                let req_handler = router.match_route(request.http_method(), request.url());

                if req_handler.is_none() {
                    sender::send_response(stream, Response::not_found());
                    return;
                }
                let req_handler = req_handler.unwrap();
                let resp = req_handler.handle_request(&request);
                sender::send_response(stream, resp);
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}

#[derive(Debug)]
struct RootHandler {}
impl Handler for RootHandler {
    fn handle_request(&self, req: &Request) -> Response {
        Response::new(response::StatusCode::Ok, &String::default())
    }
}

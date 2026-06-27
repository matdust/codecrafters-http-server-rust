use std::io::{BufRead, BufReader};
use std::net::TcpListener;

use crate::handler::*;
use crate::{
    handler::Handler,
    header::HeaderName,
    request::Request,
    response::{Response, ResponseBuilder, StatusCode},
    router::Router,
};

mod handler;
mod header;
mod request;
mod response;
mod router;
mod sender;

const PORT: u16 = 4221;

fn main() {
    let router = Router::default();

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

                let mut request = Request::parse(payload);
                let req_handler = router.match_route(request.http_method(), request.url());

                if req_handler.is_none() {
                    sender::send_response(stream, Response::not_found());
                    return;
                }
                let (req_handler, params) = req_handler.unwrap();
                request.params = params;
                let resp = req_handler.handle_request(&request);
                sender::send_response(stream, resp);
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}

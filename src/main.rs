use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use std::net::TcpListener;

use crate::{
    handler::Handler,
    request::Request,
    response::{Response, StatusCode},
    router::Router,
    sender::send_response,
};

mod handler;
mod request;
mod response;
mod router;
mod sender;

const PORT: u16 = 4221;

fn main() {
    let mut router = Router::default();

    let _ = router.add_route(request::HttpMethod::GET, "/", &RootHandler {});
    let _ = router.add_route(request::HttpMethod::GET, "/echo/{str}", &EchoHandler {});

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

#[derive(Debug)]
struct RootHandler {}
impl Handler for RootHandler {
    fn handle_request(&self, _req: &Request) -> Response {
        Response::new(response::StatusCode::Ok, &String::default())
    }
}

#[derive(Debug)]
struct EchoHandler {}

impl Handler for EchoHandler {
    fn handle_request(&self, req: &Request) -> Response {
        println!("{:?}", &req);
        match req.params.get("str") {
            Some(value) => {
                let mut resp = Response::new(StatusCode::Ok, "");
                resp.body = Some(value.clone());
                resp
            }
            None => Response::not_found(),
        }
    }
}

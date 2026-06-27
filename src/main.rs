use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use std::net::TcpListener;

use crate::{
    handler::Handler,
    request::{HeaderName, Request},
    response::{ContentType, Response, ResponseBuilder, StatusCode},
    router::Router,
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
    let _ = router.add_route(
        request::HttpMethod::GET,
        "/user-agent",
        &UserAgentHandler {},
    );

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
        ResponseBuilder::default()
            .status_code(StatusCode::Ok)
            .build()
    }
}

#[derive(Debug)]
struct EchoHandler {}

impl Handler for EchoHandler {
    fn handle_request(&self, req: &Request) -> Response {
        match req.params.get("str") {
            Some(value) => {
                let mut resp = ResponseBuilder::default()
                    .header(
                        request::HeaderName::ContentType,
                        ContentType::TextPlain.as_str(),
                    )
                    .header(request::HeaderName::ContentLength, &value.len().to_string())
                    .status_code(StatusCode::Ok)
                    .build();

                resp.body = Some(value.clone());
                resp
            }
            None => Response::not_found(),
        }
    }
}

#[derive(Debug)]
struct UserAgentHandler {}

impl Handler for UserAgentHandler {
    fn handle_request(&self, req: &Request) -> Response {
        match req.headers().get(&HeaderName::UserAgent) {
            Some(user_agent) => {
                let mut resp = ResponseBuilder::default()
                    .status_code(StatusCode::Ok)
                    .header(HeaderName::ContentType, ContentType::TextPlain.as_str())
                    .header(HeaderName::ContentLength, &user_agent.len().to_string())
                    .build();

                resp.body = Some(user_agent.clone());
                resp
            }

            None => Response::not_found(),
        }
    }
}

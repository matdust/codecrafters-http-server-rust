use std::io::{BufRead, BufReader, Read, Write};
#[allow(unused_imports)]
use std::net::TcpListener;

use crate::{handler::Handler, request::Request, router::Router};

mod handler;
mod request;
mod response;
mod router;

const PORT: u16 = 4221;

fn main() {
    println!("Server starter on {}", PORT);
    let mut router = Router::default();

    let _ = router.add_route(request::HttpMethod::GET, "/{foo}/{bar}", &Foo {});
    let _ = router.add_route(request::HttpMethod::GET, "/{echo}", &Foo {});

    let _ = router.add_route(request::HttpMethod::GET, "/foo/bar", &Foo {});
    let _ = router.add_route(request::HttpMethod::GET, "/echo", &Foo {});

    let _ = router.add_route(
        request::HttpMethod::GET,
        "/foo/{fooId}/bar/{barId}",
        &Foo {},
    );
    let _ = router.add_route(request::HttpMethod::GET, "/foo/{fooId}", &Foo {});
    let _ = router.add_route(request::HttpMethod::GET, "/foo/{fooId}/bar", &Foo {});
    println!("{:#?}", router);

    return;

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

                println!("req lines: {:?}", payload);
                let request = Request::new(payload);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

struct Foo {}
impl Handler for Foo {
    fn handle_request(&self, req: &Request) -> response::Response {
        todo!()
    }
}

impl std::fmt::Debug for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Foo handler")
    }
}

use clap::Parser;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::{args::Args, request::Request, response::Response, router::Router};

mod args;
mod handler;
mod header;
mod request;
mod response;
mod router;
mod sender;

const PORT: u16 = 4221;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4221").await?;

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(mut stream: tokio::net::TcpStream) {
    let mut reader = BufReader::new(&mut stream);
    let mut payload = Vec::new();
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line).await.unwrap();
        if n == 0 {
            break;
        }

        let trimmed = line.trim_end().to_string();
        if trimmed.is_empty() {
            break;
        }
        payload.push(trimmed);
    }

    let mut request = Request::parse(payload);
    let router = Router::global();

    let resp = match router.match_route(request.http_method(), request.url()) {
        Some((handler, params)) => {
            request.params = params;
            handler.handle_request(&request)
        }
        None => Response::not_found(),
    };

    sender::send_response(stream, resp).await;
}

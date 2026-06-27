use crate::{
    args::Args,
    header::{ContentType, HeaderName},
    request::Request,
    response::{Response, ResponseBuilder, StatusCode},
};

pub trait Handler: std::fmt::Debug + Send + Sync {
    fn handle_request(&self, req: &Request) -> Response;
}

#[derive(Debug)]
pub struct RootHandler {}
impl Handler for RootHandler {
    fn handle_request(&self, _req: &Request) -> Response {
        ResponseBuilder::default()
            .status_code(StatusCode::Ok)
            .build()
    }
}

#[derive(Debug)]
pub struct EchoHandler {}

impl Handler for EchoHandler {
    fn handle_request(&self, req: &Request) -> Response {
        match req.params.get("str") {
            Some(value) => {
                let mut resp = ResponseBuilder::default()
                    .header(HeaderName::ContentType, ContentType::TextPlain.as_str())
                    .header(HeaderName::ContentLength, &value.len().to_string())
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
pub struct UserAgentHandler {}

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

#[derive(Debug)]
pub struct FileHandler {}

impl Handler for FileHandler {
    fn handle_request(&self, req: &Request) -> Response {
        match req.params.get("filename") {
            Some(filename) => {
                match std::fs::read(format!(
                    "{}{}",
                    &Args::get().directory.clone().unwrap(),
                    filename
                )) {
                    Ok(file_content) => {
                        let mut resp = ResponseBuilder::default()
                            .status_code(StatusCode::Ok)
                            .header(HeaderName::ContentType, ContentType::OctetStream.as_str())
                            .header(HeaderName::ContentLength, &file_content.len().to_string())
                            .build();

                        match String::from_utf8(file_content.clone()) {
                            Ok(content) => {
                                resp.body = Some(content);
                                resp
                            }
                            Err(_) => Response::not_found(),
                        }
                    }
                    Err(_) => Response::not_found(),
                }
            }
            None => Response::not_found(),
        }
    }
}

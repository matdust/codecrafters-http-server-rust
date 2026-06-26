use std::io::Write;

use crate::{
    request::{HttpMethod, Request},
    response::{Response, StatusCode},
};

pub trait Handler: std::fmt::Debug {
    fn handle_request(&self, req: &Request) -> Response;
}

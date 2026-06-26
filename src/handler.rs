
use crate::{
    request::Request,
    response::Response,
};

pub trait Handler: std::fmt::Debug {
    fn handle_request(&self, req: &Request) -> Response;
}

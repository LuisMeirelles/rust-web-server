use super::{request::Request, response::Response};

pub trait Handler {
    fn handle(request: &Request) -> Response;
}

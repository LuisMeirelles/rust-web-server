use std::collections::HashMap;

use crate::core::controller::{
    handler::Handler, request::Request, response::Response, status::Status,
};

pub struct Repeater;

impl Handler for Repeater {
    fn handle(request: &Request) -> Response {
        Response {
            body: request.body.to_string(),
            status: Status::Ok,
            headers: HashMap::new(),
        }
    }
}

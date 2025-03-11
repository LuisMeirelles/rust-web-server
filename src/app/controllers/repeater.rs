use std::collections::HashMap;

use crate::core::{request::Request, response::Response, status::Status};

pub struct Repeater;

impl Repeater {
    pub fn handle(request: &Request) -> Response {
        Response {
            body: request.body.to_string(),
            status: Status::Ok,
            headers: HashMap::new(),
        }
    }
}

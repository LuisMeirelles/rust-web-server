use std::collections::HashMap;

use crate::core::controller::{
    from_request::FromRequest, handler::Handler, request::Request, response::Response,
    status::Status,
};

pub struct User {
    name: String,
}

impl FromRequest for User {
    #[allow(unused_variables)]
    fn from_request(request: &Request) -> Option<Self>
    where
        Self: Sized,
    {
        Some(User {
            name: "teste".into(),
        })
    }
}

pub struct Repeater;

impl Handler<User> for Repeater {
    fn handle(user: User) -> Response {
        Response {
            body: format!("{{\"userName\": \"{}\"}}", user.name),
            status: Status::Ok,
            headers: HashMap::new(),
        }
    }
}

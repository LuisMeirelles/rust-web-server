use std::collections::HashMap;

use crate::core::{
    controller::{
        from_request::FromRequest, handler::Handler, request::Request, response::Response,
        status::Status,
    },
    serializer::json::JsonValue,
};

pub struct User {
    name: String,
    age: u8,
}

impl FromRequest for User {
    #[allow(unused_variables)]
    fn from_request(request: &Request) -> Option<Self>
    where
        Self: Sized,
    {
        Some(User {
            name: "teste".into(),
            age: 21,
        })
    }
}

pub struct Index;

impl Handler<User> for Index {
    fn handle(user: User) -> Response {
        let mut data: HashMap<String, JsonValue> = HashMap::new();

        data.insert("name".into(), JsonValue::String(user.name.clone()));
        data.insert("age".into(), JsonValue::Int(user.age.into()));

        Response::new(data, Status::Ok, HashMap::new())
    }
}

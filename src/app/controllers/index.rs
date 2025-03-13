use std::collections::HashMap;

use crate::core::{
    controller::{
        handler::{FromRequest, Handler},
        request::Request,
        response::{Response, Status},
    },
    serializer::json::JsonValue,
};

// start mock block
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
// end mock block

pub struct Index;

impl Handler<User> for Index {
    fn handle(user: User) -> Response {
        let mut data: HashMap<String, JsonValue> = HashMap::new();

        data.insert("name".into(), JsonValue::String(user.name.clone()));
        data.insert("age".into(), JsonValue::Int(user.age.into()));

        Response::new(data, Status::Ok, HashMap::new())
    }
}

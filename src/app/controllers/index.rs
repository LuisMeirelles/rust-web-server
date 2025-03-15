use std::collections::HashMap;

use crate::core::{
    controller::{
        handler::{FromRequest, Handler},
        request::Request,
        response::{Response, Status},
    },
    serializer::json::Json,
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
        // TODO: maybe use a ToJson trait in all serializable struct so i can pass it as generic
        // type and pass any other struct as source or something like that?
        let mut json = Json::new();

        json.set("name".into(), user.name.clone());
        json.set("age".into(), user.age as i64);

        Response::new(&json, Status::Ok, HashMap::new())
    }
}

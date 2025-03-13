use super::{request::Request, response::Response};

pub trait FromRequest {
    fn from_request(request: &Request) -> Option<Self>
    where
        Self: Sized;
}

pub trait Handler<T: FromRequest> {
    fn handle(param: T) -> Response;
}

pub fn call_handler<F, T>(handler: F, request: &Request) -> Result<Response, &str>
where
    F: Fn(T) -> Response,
    T: FromRequest,
{
    if let Some(param) = T::from_request(request) {
        Ok(handler(param))
    } else {
        Err("Invalid parameter")
    }
}

use super::request::Request;

pub trait FromRequest {
    fn from_request(request: &Request) -> Option<Self>
    where
        Self: Sized;
}

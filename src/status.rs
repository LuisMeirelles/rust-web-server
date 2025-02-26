use std::fmt::Display;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Status {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            Status::Ok => "OK",
            Status::BadRequest => "Bad Request",
            Status::NotFound => "Not Found",
            Status::InternalServerError => "Internal Server Error",
        };

        write!(f, "{description}")
    }
}

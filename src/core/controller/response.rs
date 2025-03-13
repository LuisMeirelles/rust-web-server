use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::core::serializer::json::{JsonSerializer, JsonValue};

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

pub struct Response {
    pub body: String,
    pub status: Status,
    pub headers: HashMap<String, String>,
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let status_line = self.build_status_line();

        let headers = self.build_headers();

        write!(f, "{status_line}\r\n{headers}\r\n\r\n{}", self.body)
    }
}

impl Response {
    pub fn new(
        body: HashMap<String, JsonValue>,
        status: Status,
        headers: HashMap<String, String>,
    ) -> Self {
        let body = JsonSerializer::serialize(body);

        Self {
            body,
            status,
            headers,
        }
    }

    fn build_status_line(&self) -> String {
        const HTTP_VERSION: &str = "HTTP/1.1";

        let reason = self.status.to_string();
        let status_code = self.status as u32;

        format!("{HTTP_VERSION} {status_code} {reason}")
    }

    fn build_headers(&self) -> String {
        let mut headers = self.headers.clone();

        let content_length = self.body.len().to_string();

        headers.insert("Content-Length".to_string(), content_length.to_string());

        headers
            .into_iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect::<Vec<String>>()
            .join("\r\n")
    }
}

use crate::core::status::Status;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};

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

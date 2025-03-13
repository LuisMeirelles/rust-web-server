use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use std::str::FromStr;

type Headers = HashMap<String, String>;

pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "DELETE" => Ok(Method::Delete),
            _ => Err(()),
        }
    }
}

impl Method {
    pub fn as_str(&self) -> &str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
        }
    }
}

#[allow(dead_code)]
pub struct Request {
    pub body: String,
    pub path: String,
    pub method: Method,
    pub headers: Headers,
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}\n{}",
            self.method.as_str(),
            self.path,
            self.headers
                .iter()
                .map(|(key, value)| format!("{key}: {value}"))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Request {
    pub fn new(buf_reader: &mut BufReader<&TcpStream>) -> Request {
        let request_lines: Vec<String> = buf_reader
            .by_ref()
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let mut parts = request_lines[0].split_whitespace();

        let method = Method::from_str(parts.next().unwrap()).unwrap();
        let path = parts.next().unwrap().to_string();

        let mut headers: Headers = Headers::new();

        for line in &request_lines[1..] {
            let [key, value]: [&str; 2] = line
                .split_once(":")
                .map(|(key, value)| [key.trim(), value.trim()])
                .unwrap();

            headers.insert(key.to_string(), value.to_string());
        }

        let content_length: usize = headers
            .get("Content-Length")
            .and_then(|a| a.parse::<usize>().ok())
            .unwrap_or(0);

        let mut body: Vec<u8> = vec![0; content_length];

        buf_reader.read_exact(&mut body).unwrap();

        Request {
            body: String::from_utf8_lossy(&body).to_string(),
            path,
            method,
            headers,
        }
    }
}

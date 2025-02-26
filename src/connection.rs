use crate::address::Address;
use crate::renderer::print_request_and_response;
use crate::response::Response;
use crate::status::Status;

use std::collections::HashMap;
use std::ops::Index;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub struct Connection;

impl Connection {
    pub fn listen(&self, address: Address) {
        let listener = TcpListener::bind(address.to_string()).expect("Failed to bind to address");

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("\nConnection established!");

            Connection::handle_connection(stream);
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&stream);

        let http_request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let filename = http_request
            .index(0)
            .split(" ")
            .nth(1)
            .and_then(|path| path.strip_prefix("/"))
            .filter(|path| !path.is_empty())
            .unwrap_or("index.html");

        let file_contents = fs::read_to_string(format!("./views/{filename}"));

        let (status, body) = match file_contents {
            Ok(contents) => (Status::Ok, contents),
            Err(_) => {
                let status = Status::NotFound;
                let status_code = status as u16;

                let file_path = format!("views/errors/{}.html", status_code);

                let body = fs::read_to_string(file_path).unwrap_or(String::from("Not Found"));

                (status, body)
            }
        };

        let mut headers: HashMap<String, String> = HashMap::new();

        headers.insert("Content-Type".to_string(), "text/html".to_string());

        let response = Response {
            body,
            status,
            headers,
        };

        print_request_and_response(&http_request, &response);

        stream.write_all(response.to_string().as_bytes()).unwrap();
    }
}

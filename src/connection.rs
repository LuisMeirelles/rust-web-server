use crate::address::Address;
use crate::response::Response;
use crate::renderer::print_request_and_response;
use crate::request::Request;

use crate::status::Status;
use std::collections::HashMap;
use std::{
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
        let mut buf_reader = BufReader::new(&stream);

        let mut headers: HashMap<String, String> = HashMap::new();

        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let request = Request::new(&mut buf_reader);

        let response = Response {
            body: (&request.body).to_string(),
            status: Status::Ok,
            headers,
        };

        print_request_and_response(&request, &response);

        stream.write_all(response.to_string().as_bytes()).unwrap();
    }
}

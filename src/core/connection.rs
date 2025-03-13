use crate::core::address::Address;

use crate::app::controllers::Index;

use crate::core::loggers::renderer::print_request_and_response;

use std::collections::HashMap;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use super::controller::handler::{call_handler, Handler};
use super::controller::request::Request;

pub struct Connection;

impl Connection {
    pub fn listen(address: Address) {
        let listener = TcpListener::bind(address.to_string()).expect("Failed to bind to address");

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("\nConnection established!");

            Connection::handle_connection(stream);
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buf_reader = BufReader::new(&stream);

        let request = Request::new(&mut buf_reader);

        // TODO: add the router
        let mut response = call_handler(Index::handle, &request).unwrap();

        // TODO: make a type for Headers
        let mut headers: HashMap<String, String> = HashMap::new();

        headers.insert("Content-Type".to_string(), "application/json".to_string());

        response.headers.extend(headers);

        // TODO: add post controller actions
        // TODO: create a pipeline abstraction for middlewares/validators/transformers -> controllers -> interceptors
        print_request_and_response(&request, &response);

        stream.write_all(response.to_string().as_bytes()).unwrap();
    }
}

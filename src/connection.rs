use crate::response::Response;
use crate::status::Status;
use crate::address::Address;

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

pub struct Connection;

impl Connection {
    pub fn listen(&self, address: Address) {
        let listener = TcpListener::bind(address.to_string()).expect("Failed to bind to address");

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("Connection established!\n");

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

        println!("Request:\n");
        println!("{}\n\n", http_request.join("\r\n"));

        let contents = "<div style=\"width: 100px; height: 100px; background: red;\"></div>";

        let mut headers: HashMap<String, String> = HashMap::new();

        headers.insert("Content-Type".to_string(), "text/html".to_string());

        let response = Response {
            contents: contents.to_string(),
            status: Status::Ok,
            headers,
        };

        println!("Response: \n\n{}", response.to_string());

        stream.write_all(response.to_string().as_bytes()).unwrap();
    }
}

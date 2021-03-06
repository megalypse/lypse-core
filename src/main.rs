use std::io::prelude::*;
use std::net::TcpListener;

use lypse_core::parser::default_parser::DefaultParser;
use lypse_core::parser::parser::RequestParser;

fn main() {
    let parser = DefaultParser {};

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for incoming_connection in listener.incoming() {
        let mut stream = incoming_connection.unwrap();

        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        let request_string = String::from_utf8_lossy(&buffer);

        let result = parser.parse(&request_string);

        println!("{:?}", result);
    }
}

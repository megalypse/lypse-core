use std::io::prelude::*;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for incoming_connection in listener.incoming() {
        let mut stream = incoming_connection.unwrap();

        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        let parsed_request = String::from_utf8_lossy(&buffer);
        println!("{}", parsed_request);
    }
}

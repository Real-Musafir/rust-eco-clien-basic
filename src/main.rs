use std::io::prelude::*;
use std::net::TcpStream;

const ECHO_SERVER_ADDRESS: &str = "localhost:1234";

fn main() {
    // connection
    println!("Connecting to {}", ECHO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS) {
        println!("connected to echo server {}", ECHO_SERVER_ADDRESS);
    } else {
        println!("failed to connect to echo server {}", ECHO_SERVER_ADDRESS);
    }
}

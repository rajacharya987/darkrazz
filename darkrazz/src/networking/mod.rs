use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

pub fn initialize_network() {
    println!("Starting TCP server...");

    // Start the server on port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    println!("Received request: {}", String::from_utf8_lossy(&buffer[..]));

    let response = "Hello from DarkRazz!";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


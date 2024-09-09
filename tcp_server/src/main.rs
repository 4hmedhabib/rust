use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // this is a buffer to read data from the client
    let mut buffer = [0; 1024];
    // reads data from the stream and stores it in the buffer
    stream.read(&mut buffer).expect("Failed to read from client");
    // converts the data in the buffer into a UTF-8 encoded string.
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", request);
    let response = "Hello, client!".as_bytes();
    stream.write(response).expect("Failed to write response!");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind address!");
    println!("Server running on 127.0.0.1:8080");

    for stream in listener.incoming(){
        match stream {
            Ok( tcp_stream) => {
                // Spawn a new thread to handle the client connection
                std::thread::spawn(|| handle_client( tcp_stream));
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}

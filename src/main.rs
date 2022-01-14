use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // bind returns a Result<T, E>

    // incoming() returns an iterator that gives us a sequence of streams
    // one stream = one connection client/server
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

}

// TcpStream keeps track of the data returned. Its internal state might change so it needs to be mutable
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n"; // adding the b"" syntax transforms get into a byte string 

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    }

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}\r\n\r\nContent-Length: {}\r\n\r\n{}", 
        status_line,
        contents.len(), 
        contents
    );
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
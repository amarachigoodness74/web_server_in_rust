use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "8081";
    let end_point: String = HOST.to_owned() + ":" + PORT;
    let listener = TcpListener::bind(end_point).unwrap();
    println!("Web server is listening on port: {}", PORT);

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                handle_connection(_stream);
            }
            Err(e) => panic!("Encountered IO error: {e}"),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Read request
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Connection established!!! \n ");

    let home_page = b"GET / HTTP/1.1\r\n";
    let hello_page = b"GET /hello HTTP/1.1\r\n";
    // Send back a response
    let (status_line, filename) = if buffer.starts_with(home_page) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buffer.starts_with(hello_page) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open("pages/".to_owned() + filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

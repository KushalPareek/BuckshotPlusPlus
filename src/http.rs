use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use log::{debug, info};

fn handle_read(mut stream: &TcpStream, debug: u8) {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => match debug {
            1 => {
                let req_str = String::from_utf8_lossy(&buf);
                debug!("{}", req_str);
            }
            _ => {}
        },
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    match stream.write(response) {
        Ok(_) => info!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream, debug: u8) {
    handle_read(&stream, debug);
    handle_write(stream);
}

pub fn main(port: i16, debug: u8) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    info!("Listening for connections on port {}", port);
    info!("http://127.0.0.1:{}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream, debug));
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}

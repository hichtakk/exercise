// https://ayende.com/blog/176577/first-run-with-rust-the-echo-server

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Unable to bind to socket");
    let addr = listener.local_addr().expect("unable to get the local port?");

    println!("Listening on port {}", addr.port());

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => panic!(e),
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 16];
    if let Err(_) = stream.write("Hello from Rust".as_bytes()) {
        return;
    }
    println!("connection accepted");

    loop {
        if let Ok(read) = stream.read(&mut buffer) {
            if read == 0 {
                break;
            }
            if let Err(_) = stream.write(&buffer[0..read]) {
                break;
            }
        } else {
            break;
        }
    }
    println!("disconnected");
}


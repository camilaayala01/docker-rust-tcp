use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::net::{ToSocketAddrs};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                false
            } else {
                println!("Received: {}", String::from_utf8_lossy(&buffer[..size]));
                stream.write_all(b"Message received").unwrap();
                true
            }
        }
        Err(_) => {
            println!("Connection closed.");
            false
        }
    } {}
}

fn main() -> std::io::Result<()> {
    let addr = "tcp-server:8080"
        .to_socket_addrs()?
        .next()
        .ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Failed to resolve address"))?;
    
    let listener = TcpListener::bind(addr)?;
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    Ok(())
}

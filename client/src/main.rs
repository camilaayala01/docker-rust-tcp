use std::net::TcpStream;
use std::io::{Write, Read};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("tcp-server:8080")?;
    println!("Connected to the server!");

    let message = "Hello from client!";
    stream.write_all(message.as_bytes())?;
    println!("Sent: {}", message);

    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer)?;
    println!("Received: {}", String::from_utf8_lossy(&buffer[..size]));

    Ok(())
}

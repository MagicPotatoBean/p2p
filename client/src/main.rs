use std::{io::Read, net::TcpStream};

fn main() {
    println!("Hello, world!");
    let mut client = TcpStream::connect("127.0.0.5:80").unwrap();
    let mut buf = [0u8; 20];
    client.read(&mut buf).unwrap();
    println!("I am {}", client.local_addr().unwrap());
    println!("Got other client: {}", String::from_utf8_lossy(&buf));
}

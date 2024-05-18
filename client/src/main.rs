use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    ptr::copy_nonoverlapping,
};

fn main() {
    println!("Hello, world!");
    let mut client = TcpStream::connect("35.177.171.124:80").unwrap();
    let mut buf = [0u8; 20];
    let bytes_read = client.read(&mut buf).unwrap();
    let remote: SocketAddr = String::from_utf8_lossy(&buf[1..bytes_read])
        .parse()
        .unwrap();
    client.shutdown(std::net::Shutdown::Both);
    println!("Attempting a connection to {}", remote);
    let host_mode = buf[0].eq(&b'h');
    if host_mode {
        let mut puncher = TcpStream::connect(remote).unwrap();
        puncher.shutdown(std::net::Shutdown::Both).unwrap();
        let mut listener = TcpListener::bind(remote).unwrap();
        let (mut connection, address) = listener.accept().unwrap();
        connection.write_all(b"Test").unwrap();
    } else {
        let mut connection = TcpStream::connect(remote).unwrap();
        let mut buf = [0u8; 4];
        connection.read_exact(&mut buf).unwrap();
        println!("Received message: {}", String::from_utf8_lossy(&buf));
    }
}

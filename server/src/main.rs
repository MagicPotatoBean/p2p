use std::{
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    sync::mpsc::{Receiver, Sender},
    thread::spawn,
    time::Duration,
};
const ADDRESS: SocketAddr = SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 80);
fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind(ADDRESS).expect("failed to create TcpListener");
    listener_loop(listener)
}
fn listener_loop(listener: TcpListener) -> ! {
    let (tx, rx): (
        Sender<(TcpStream, SocketAddr)>,
        Receiver<(TcpStream, SocketAddr)>,
    ) = std::sync::mpsc::channel();
    spawn(move || client_connector(rx));
    for client in listener.incoming().flatten() {
        if let Ok(remote_address) = client.peer_addr() {
            println!("{remote_address} requested an introduction!");
            tx.send((client, remote_address)).expect("Channel closed");
        }
    }
    unreachable!()
}
fn client_connector(requests: Receiver<(TcpStream, SocketAddr)>) -> ! {
    let mut last_request = Some(requests.recv().expect("Channel closed"));
    loop {
        let (mut stream_two, address_two) = requests.recv().expect("Channel closed");
        match last_request {
            Some((ref mut stream_one, address_one)) => {
                if stream_two
                    .write_all(format!("h{}", address_one).as_bytes())
                    .is_err()
                {
                    println!("Failed to write to {address_two}, so {address_one} was sent an erroneous address.");
                    last_request = None;
                    continue;
                } else if stream_one
                    .write_all(format!("c{}", address_two).as_bytes())
                    .is_err()
                {
                    println!("Failed to write to {address_one}, dropping their request.");
                    last_request = None;
                    continue;
                } else {
                    println!("Succesfully introduced {address_one} to {address_two}");
                }
            }
            None => {
                last_request = Some((stream_two, address_two));
            }
        }
    }
}

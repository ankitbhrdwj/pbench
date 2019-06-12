extern crate nix;
extern crate util;

use nix::unistd::{fork, ForkResult};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::FromStr;

use self::util::config::{load_config, MyConfig};

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 128]; // TODO: Decide on the payload size.
    while match stream.read(&mut data) {
        Ok(size) => {
            if size == 0 {
                stream.shutdown(Shutdown::Both).unwrap();
                return;
            }
            stream.write(&data[0..size]).unwrap();
            true
        }

        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn listen(listener: &TcpListener) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                //println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }

            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn main() {
    let mut start_port = 1024;
    let config: MyConfig = load_config();
    let ip = Ipv4Addr::from_str(&config.server).unwrap();
    for _i in 0..config.ports {
        let socket_addr: SocketAddr = SocketAddr::new(IpAddr::V4(ip), start_port);
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                println!("New process with pid: {} for port {}", child, start_port);
            }

            Ok(ForkResult::Child) => loop {
                let listener = TcpListener::bind(socket_addr).unwrap();
                listen(&listener);
                drop(listener);
            },

            Err(_) => println!("Fork failed"),
        }
        start_port += 1;
    }
    println!("Initilization done for {} port(s)", config.ports);
    loop {}
}

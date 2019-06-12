extern crate confy;
extern crate nix;
#[macro_use]
extern crate serde_derive;

use nix::unistd::{fork, ForkResult};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    server: String,
    ports: u64,
}

impl Default for MyConfig {
    fn default() -> Self {
        MyConfig {
            server: "10.10.1.1".to_string(),
            ports: 1,
        }
    }
}

fn load() -> MyConfig {
    let config = confy::load("../config");
    match config {
        Ok(config) => config,
        Err(err) => {
            println!("{}. Taking default config.", err);
            MyConfig::default()
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 128]; // TODO: Decide on the payload size.
    while match stream.read(&mut data) {
        Ok(size) => {
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
                println!("New connection: {}", stream.peer_addr().unwrap());
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
    let config = load();
    let ip = Ipv4Addr::from_str(&config.server).unwrap();
    println!("{}", config.ports);
    for _i in 0..config.ports {
        let socket_addr: SocketAddr = SocketAddr::new(IpAddr::V4(ip), start_port);
        let listener = TcpListener::bind(socket_addr).unwrap();
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                println!("New process with pid: {} for port {}", child, start_port);
            }

            Ok(ForkResult::Child) => {
                listen(&listener);
                drop(listener);
            }

            Err(_) => println!("Fork failed"),
        }
        start_port += 1;
    }
    println!("Initilization done!!!");
    loop {}
}

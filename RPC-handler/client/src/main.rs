extern crate core_affinity;
extern crate util;

use std::io::{Read, Write};
use std::net::Shutdown;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::from_utf8;
use std::str::FromStr;

use util::config::*;
use util::cycles::*;

struct Client {
    pub config: MyConfig,
    pub connections: Vec<TcpStream>,
    pub recvd: u64,
    pub start: u64,
    pub stop: u64,
}

impl Client {
    pub fn new() -> Client {
        Client {
            config: load_config(),
            connections: Vec::with_capacity(65535),
            recvd: 0,
            start: 0,
            stop: 0,
        }
    }

    pub fn send_recv(&mut self) {
        self.start = rdtsc();
        loop {
            let mut stream = &self.connections[0];
            let msg = b"Hello!";
            stream.write(msg).unwrap();

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        self.recvd += 1;
                        if self.recvd >= 1000000 {
                            self.stop = rdtsc();
                            stream.shutdown(Shutdown::Both).unwrap();
                            break;
                        }
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                }

                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        println!(
            "Throughput {}",
            self.recvd as f64 / to_seconds(self.stop - self.start)
        );
    }
}

fn main() {
    let mut client = Client::new();
    let mut start_port = 1024;
    let ip = Ipv4Addr::from_str(&client.config.server).unwrap();
    for _i in 0..client.config.ports {
        let socket_addr: SocketAddr = SocketAddr::new(IpAddr::V4(ip), start_port);
        match TcpStream::connect(socket_addr) {
            Ok(mut stream) => {
                client.connections.push(stream);
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
        start_port += 1;
    }

    // Make it multi-threaded.
    client.send_recv();
}

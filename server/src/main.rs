use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 128]; // TODO: Decide on the payload size.
    while match stream.read(&mut data) {
        Ok(size) => {
	    println!("{}", size);
            stream.write(&data[0..size]).unwrap();
            true
        },

        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("10.10.1.1:3333").unwrap();
    println!("Server listening on port 3333");
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
    // close the socket server
    drop(listener);
    println!("Hello, world!");
}

mod config;

use libc::fork;
use std::{
    io::Read,
    net::{Ipv4Addr, TcpListener, TcpStream},
};

/// Receives data from a TCP Stream.
fn handle_connection(mut stream: TcpStream) {
    println!("Connection Opened");

    let mut buf = Vec::new();
    match stream.read_to_end(&mut buf) {
        Ok(_) => {
            println!("Received: {}", String::from_utf8_lossy(&buf));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!("Connection Closed");
}

/// Binds to port, forks on connections.
fn main() {
    let conf = config::read_config();

    // TODO: get port form env/config
    let listener = TcpListener::bind((Ipv4Addr::new(127, 0, 0, 1), conf.port))
        .expect("failed to bind to port");

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                // SAFETY: parent process immediately closes file descriptor
                //         child process immediately drops listener
                let child = unsafe { fork() };
                match child {
                    -1 => {
                        panic!();
                    }
                    0 => {
                        drop(listener);
                        handle_connection(stream);
                        break;
                    }
                    _ => {}
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

use libc::fork;
use std::{
    io::{Read, Write},
    net::{Ipv4Addr, TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut buf = [0; 0x100];
        match stream.read(&mut buf) {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e);
            }
        }
        match stream.write(&buf) {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}

fn main() {
    // TODO: get port form env/config
    let port = 1337;
    let listener =
        TcpListener::bind((Ipv4Addr::new(127, 0, 0, 1), port)).expect("failed to bind to port");

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

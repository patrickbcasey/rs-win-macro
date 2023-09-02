use rdev::{  listen, Event };

if let Err(error) = listen(callback) {
    println!("Error: {:?}", error);
}

fn callback(event: Event) {
    println!("Event: {:?}", event);
    match event.name {
      Some(string) => println!("Event name: {:?}", string),
      None => ()
    }
}


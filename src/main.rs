use rdev::{  listen, Event, EventType, Key::*};
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use std::{ thread, time:: SystemTime, path::PathBuf };
use maco::keyhook::simulate::simulate;
use std::fs::{ OpenOptions, read_to_string };
use std::io::Write;
use clap::{arg, command, Arg, value_parser, ArgAction, Parser };
use std::time;
use maco::keyhook::helper::match_string_to_key;
use maco::keyhook::keycodes::code_from_key;
use rand::prelude::*;


#[derive(Parser)]
struct Cli {
    #[clap(short, long, default_value = "run")]
    pattern: String,
    path: std::path::PathBuf
}

fn main() {
    let matches = command!()
        .arg(Arg::new("mode").short('m').long("rec").action(ArgAction::SetFalse))
        .arg(
            arg!(-f --file [FILE] "Sets a file path")
            .required(false)
            .value_parser(value_parser!(PathBuf))
    ).get_matches();

    let path: PathBuf = matches.get_one::<PathBuf>("file").unwrap_or(&PathBuf::from("./data.txt")).to_path_buf();

    match matches.get_one::<bool>("mode") {
        Some(true) => {
            let move_list = build_move_list(path); 
            loop {
                if unsafe { GetAsyncKeyState(i32::from(code_from_key(Num0).unwrap())) } < 0 {
                    println!("key pressed");
                    for i in move_list.iter() {
                        if unsafe { GetAsyncKeyState(i32::from(code_from_key(Num9).unwrap())) } < 0 {
                            println!("Breaking out");
                            break;
                        }
                        send(&i.event_type);
                        let y: u64 = (random::<f64>() * 11.0) as u64;
                        match random::<bool>() {
                            true => thread::sleep(i.time + time::Duration::from_millis(y)),
                            false => thread::sleep(i.time - time::Duration::from_millis(y)),
                        };
                    }
                    break;
                }
            }
        },
        Some(false) => {
            print!("recording");
            loop {
                if unsafe { GetAsyncKeyState(i32::from(code_from_key(Num0).unwrap())) } < 0 {
                    recording(path);
                    break;
                }
            }
        }
        _ => panic!("Not implemented")
    }

        
    // thread::sleep(time::Duration::from_millis(3000));
    // recording(PathBuf::from("./data2.txt"));
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

fn build_move_list(file_string: PathBuf) -> Vec<Move> {
    let instructions = read_to_string(file_string).expect("cannot read file");
    let moves_string = instructions.trim().replace(')', "");
    let moves: Vec<&str> = moves_string.split("\r\n").collect::<Vec<&str>>().into_iter().flat_map(|x| x.split(&[',', '('])).collect();
    let grouping = moves.chunks(3).into_iter();
    let timers: Vec<Move> = grouping.map(|x| {
        match x[0] {
            "KeyPress" => Move{ event_type: EventType::KeyPress(match_string_to_key(x[1])), time: time::Duration::from_millis(x[2].parse::<u64>().unwrap()) },
            "KeyRelease" => Move{ event_type:EventType::KeyRelease(match_string_to_key(x[1])), time: time::Duration::from_millis(x[2].parse::<u64>().unwrap()) },
            _ => panic!("Not implemented")
        }
    }).collect::<Vec<Move>>();

    let mut move_list: Vec<Move> = Vec::new();
    
    for i in 0..timers.len() {
        if i == 0 {
            move_list.push(timers[i]);
            continue;
        }
        move_list.push(Move { event_type: timers[i].event_type, time: timers[i].time - timers[i-1].time });
    }   
    
    println!("{:?}", move_list);
    move_list
}


fn recording(file_string: PathBuf) {
    let mut data_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_string)
        .expect("cannot open file");
    let clock = SystemTime::now();

    let closure = {
        move |event: Event| {
        
        match event.event_type {
            EventType::KeyPress(key) => {
                data_file.write_all(format!("{:?},{:?}\n", event.event_type, event.time.duration_since(clock).unwrap().as_millis()).as_bytes()).expect("cannot write to file");
            },
            EventType::KeyRelease(key) => {
                data_file.write_all(format!("{:?},{:?}\n", event.event_type, event.time.duration_since(clock).unwrap().as_millis()).as_bytes()).expect("cannot write to file");
            },
            _ => (),
        }
    }};

    if let Err(error) = listen(closure) {
        println!("Error: {:?}", error)
    }
}

#[derive(Debug, Clone, Copy )]
struct Move {
    time: time::Duration,
    event_type: EventType,
}
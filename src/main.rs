mod input;

use std::fs::{File, OpenOptions, self};
use std::io::prelude::*;
use input::send_key_event;
#[allow(unused_variables)]


use windows::Win32::UI::Input::KeyboardAndMouse::{ GetAsyncKeyState, INPUT, INPUT_0, INPUT_TYPE, KEYBDINPUT, KEYBD_EVENT_FLAGS, SendInput, VIRTUAL_KEY, VK_NUMPAD0};
use windows::Win32::UI::WindowsAndMessaging::{ FindWindowA, ShowWindow, SHOW_WINDOW_CMD };
use windows::Win32::System::Console::AllocConsole;
use std::{thread, time};

fn main() {
    stealth();
    let input: String = fs::read_to_string("./input.txt")
        .expect("Unable to read file");

        thread::sleep(time::Duration::from_millis(3000));
        send_key_event(2, 0);

    loop {
        if unsafe { GetAsyncKeyState(0x31) >> 15 & 1 == 1 } {
            break;
        }
        
        if unsafe { GetAsyncKeyState(0x30) >> 15 & 1 == 1 } {
            let instructions: Vec<u16> = input.clone().trim().split(',').map(|x| u16::from_str_radix(x.trim_start_matches("0x"), 16).unwrap()).collect();
            run_maco(instructions);
        }


        if unsafe { GetAsyncKeyState(0x35) >> 15 & 1 == 1 } {
            record_maco();

        }

        thread::sleep(time::Duration::from_millis(1000));
    }
}


fn run_maco(instructions: Vec<u16>) {
    loop {
        for i in instructions.clone() {
            if unsafe { GetAsyncKeyState(0x39) } >> 15 & 1 == 1 {
                return;
            }

            println!("{:?}", unsafe { GetAsyncKeyState(0x39) >> 15 & 1 });
            send_key_event(i, 0);
            thread::sleep(time::Duration::from_millis(1000));
        }
    }
}

fn record_maco() {
    loop {
        if unsafe { GetAsyncKeyState(0x36) } >> 15 & 1 == 1 {
            break;
        }
                    
           
        thread::sleep(time::Duration::from_millis(1000));
        
    }
}

fn stealth() {
    let mut stealth: windows::Win32::Foundation::HWND;
    unsafe {
        AllocConsole();
        stealth = FindWindowA(windows::core::PCSTR::from_raw(std::ffi::CString::new("ConsoleWindowClass").unwrap().as_ptr() as *const u8), None);
        ShowWindow(stealth,SHOW_WINDOW_CMD(0));
    }
}
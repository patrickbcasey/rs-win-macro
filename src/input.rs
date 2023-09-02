use windows::Win32::UI::Input::KeyboardAndMouse::{ INPUT, INPUT_0, INPUT_TYPE, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, SendInput, VIRTUAL_KEY, KEYEVENTF_SCANCODE};
use std::{ thread, time };

pub fn send_key_event(scan_code: u16, flags: u32) {
  let input  = INPUT {
    r#type: INPUT_TYPE {0: 1},
    Anonymous: INPUT_0 {
        ki: KEYBDINPUT {
            wVk: VIRTUAL_KEY(0),
            wScan: scan_code,
            // dwFlags: KEYBD_EVENT_FLAGS(flags),
            // dwFlags: if up { KEYEVENTF_KEYUP } else { KEYBD_EVENT_FLAGS(0) },
            dwFlags: KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP,
            time: 0,
            dwExtraInfo: 0,
        },
    },
  };
  
  unsafe {
    SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    thread::sleep(time::Duration::from_millis(1000));
    // SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    println!("Sent key event: {:?}", input.Anonymous.ki.dwFlags);
  }
}

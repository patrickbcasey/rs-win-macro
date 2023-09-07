use rdev::{ EventType, SimulateError, Key };
use std::mem::size_of;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{DWORD, UINT, WORD};
use winapi::um::winuser::{
    INPUT_u, SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT,
    KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE,
};

pub fn sim_keyboard_event(flags: DWORD, vk: WORD, scan: WORD) -> Result<(), SimulateError> {
  let mut union: INPUT_u = unsafe { std::mem::zeroed() };
  let inner_union = unsafe { union.ki_mut() };
  *inner_union = KEYBDINPUT {
      wVk: vk,
      wScan: scan,
      dwFlags: flags,
      time: 0,
      dwExtraInfo: 0,
  };
  let mut input = [INPUT {
      type_: INPUT_KEYBOARD,
      u: union,
  }; 1];
  let value = unsafe {
      SendInput(
          input.len() as UINT,
          input.as_mut_ptr(),
          size_of::<INPUT>() as c_int,
      )
  };
  if value != 1 {
      Err(SimulateError)
  } else {
      Ok(())
  }
}

pub fn simulate(event_type: &EventType) -> Result<(), SimulateError> {
  match event_type {
      EventType::KeyPress(key) => {
          let code = code_from_key(*key).ok_or(SimulateError)?;
          sim_keyboard_event(KEYEVENTF_SCANCODE, 0, code)
      }
      EventType::KeyRelease(key) => {
          let code = code_from_key(*key).ok_or(SimulateError)?;
          sim_keyboard_event(KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP, 0, code)
      }
      _ => Ok(()),

  }
}


fn code_from_key(key: Key) -> Option<u16> {
  match key {
      Key::Alt => Some(56),
      Key::ShiftLeft => Some(42),
      Key::DownArrow => Some(80),
      Key::LeftArrow => Some(75),
      Key::RightArrow => Some(77),
      Key::UpArrow => Some(72),
      Key::KeyA => Some(30),
      Key::KeyB => Some(48),
      Key::KeyC => Some(46),
      Key::KeyD => Some(32),
      Key::KeyE => Some(18),
      Key::KeyF => Some(33),
      Key::KeyG => Some(34),
      Key::KeyH => Some(35),
      Key::KeyI => Some(23),
      Key::KeyJ => Some(36),
      Key::KeyK => Some(37),
      Key::KeyL => Some(38),
      Key::KeyM => Some(50),
      Key::KeyN => Some(49),
      Key::KeyO => Some(24),
      Key::KeyP => Some(25),
      Key::KeyQ => Some(16),
      Key::KeyR => Some(19),
      Key::KeyS => Some(31),
      Key::KeyT => Some(20),
      Key::KeyU => Some(22),
      Key::KeyV => Some(47),
      Key::KeyW => Some(17),
      Key::KeyX => Some(45),
      Key::KeyY => Some(21),
      Key::KeyZ => Some(44),
      Key::F1 => Some(59),
      Key::F2 => Some(60),
      Key::F3 => Some(61),
      Key::F4 => Some(62),
      Key::F5 => Some(63),
      Key::F6 => Some(64),
      Key::F7 => Some(65),
      Key::F8 => Some(66),
      Key::F9 => Some(67),
      Key::F10 => Some(68),
      Key::F11 => Some(69),
      Key::Delete => Some(83),
      Key::ControlLeft => Some(29),
      _ => None,
  }
}
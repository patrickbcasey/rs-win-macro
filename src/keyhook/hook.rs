use std::{
  ffi::c_int,
  mem::MaybeUninit,
  ptr::null_mut,
  sync::atomic::{ AtomicPtr, Ordering },
};
use once_cell::sync::Lazy;
use crate::keyhook::structs::*;


use windows::Win32::{
  Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
            CallNextHookEx, GetMessageW, SetWindowsHookExW,
            UnhookWindowsHookEx, HHOOK, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL,
            WINDOWS_HOOK_ID, WM_KEYDOWN, WM_SYSKEYDOWN,
        },
};

const NULL_HHOOK: HHOOK = HHOOK(0);
static KEYBD_HHOOK: Lazy<AtomicPtr<HHOOK>> = Lazy::new(AtomicPtr::default);

pub fn handle_inputs() {
  set_hook(WH_KEYBOARD_LL, &KEYBD_HHOOK, keybd_proc);
  let mut msg: MSG = unsafe { MaybeUninit::zeroed().assume_init() };
  unsafe { GetMessageW(&mut msg, HWND(0), 0, 0) };
}




unsafe extern "system" fn keybd_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
  todo!("Handle keybinds");
  if KEYBD_BINDS.lock().unwrap().is_empty() {
    unset_hook(&KEYBD_HHOOK);
  } else if w_param.0 as u32 == WM_KEYDOWN || w_param.0 as u32 == WM_SYSKEYDOWN {
    let kb = *(l_param.0 as *const KBDLLHOOKSTRUCT);
    let scan = kb.scanCode;

  }
  // Hook Procedure

  CallNextHookEx(NULL_HHOOK, code, w_param, l_param)
}

fn set_hook(
  hook_id: WINDOWS_HOOK_ID,
  hook_ptr: &AtomicPtr<HHOOK>,
  hook_proc: unsafe extern "system" fn(c_int, WPARAM, LPARAM) -> LRESULT,
) {
  hook_ptr.store(
    unsafe { &mut SetWindowsHookExW(hook_id, Some(hook_proc), HINSTANCE(0), 0) .unwrap() },
    Ordering::Relaxed,
   );
}

fn unset_hook(hook_ptr: &AtomicPtr<HHOOK>) {
  unsafe { UnhookWindowsHookEx(*hook_ptr.load(Ordering::Relaxed)).unwrap(); }
  hook_ptr.store(null_mut(), Ordering::Relaxed);
}
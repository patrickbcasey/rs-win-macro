mod input;
use windows::Win32::UI::Input::KeyboardAndMouse::{ INPUT, INPUT_0, INPUT_TYPE, KEYBDINPUT, KEYBD_EVENT_FLAGS, SendInput, VIRTUAL_KEY, VK_NUMPAD0};

fn show_desktop() {
  let input: INPUT = {};

  inputs.INPUT_TYPE = INPUT_KEYBOARD;
  inputs.INPUT_0.ki.wVk = VK_NUMPAD0;

}
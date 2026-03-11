use crate::key::VirtualKeyboard;
use uinput::event::keyboard::Key;

pub fn open_terminal(kb: &mut VirtualKeyboard) {
    println!("Opening terminal");

    kb.combo(Key::LeftMeta, Key::Q);

    // wait for launcher
    kb.wait(1500);

    kb.type_string("terminal");

    kb.wait(500);

    kb.press(Key::Enter);

    // wait for terminal window to appear
    kb.wait(3000);
}

pub fn echo_hi(kb: &mut VirtualKeyboard) {
    println!("Typing command");

    kb.wait(500);

    kb.type_string("echo \"hi i am raj\"");

    kb.wait(200);

    kb.press(Key::Enter);
}


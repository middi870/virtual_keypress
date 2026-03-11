mod key;
mod skills;

use key::VirtualKeyboard;
use std::{thread, time::Duration};

fn main() {
    let mut keyboard = VirtualKeyboard::new();

    println!("Virtual keyboard ready");

    thread::sleep(Duration::from_secs(2));

    skills::open_terminal(&mut keyboard);

    skills::echo_hi(&mut keyboard);
}

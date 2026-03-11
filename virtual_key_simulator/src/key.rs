use std::{thread, time::Duration};
use uinput::event::keyboard::Key;

pub struct VirtualKeyboard {
    pub device: uinput::device::Device,
}

impl VirtualKeyboard {
    pub fn new() -> Self {
        let device = uinput::default()
            .unwrap()
            .name("Raj Virtual Keyboard")
            .unwrap()
            // letters
            .event(Key::A)
            .unwrap()
            .event(Key::B)
            .unwrap()
            .event(Key::C)
            .unwrap()
            .event(Key::D)
            .unwrap()
            .event(Key::E)
            .unwrap()
            .event(Key::F)
            .unwrap()
            .event(Key::G)
            .unwrap()
            .event(Key::H)
            .unwrap()
            .event(Key::I)
            .unwrap()
            .event(Key::J)
            .unwrap()
            .event(Key::K)
            .unwrap()
            .event(Key::L)
            .unwrap()
            .event(Key::M)
            .unwrap()
            .event(Key::N)
            .unwrap()
            .event(Key::O)
            .unwrap()
            .event(Key::P)
            .unwrap()
            .event(Key::Q)
            .unwrap()
            .event(Key::R)
            .unwrap()
            .event(Key::S)
            .unwrap()
            .event(Key::T)
            .unwrap()
            .event(Key::U)
            .unwrap()
            .event(Key::V)
            .unwrap()
            .event(Key::W)
            .unwrap()
            .event(Key::X)
            .unwrap()
            .event(Key::Y)
            .unwrap()
            .event(Key::Z)
            .unwrap()
            // numbers
            .event(Key::_1)
            .unwrap()
            .event(Key::_2)
            .unwrap()
            .event(Key::_3)
            .unwrap()
            .event(Key::_4)
            .unwrap()
            .event(Key::_5)
            .unwrap()
            .event(Key::_6)
            .unwrap()
            .event(Key::_7)
            .unwrap()
            .event(Key::_8)
            .unwrap()
            .event(Key::_9)
            .unwrap()
            .event(Key::_0)
            .unwrap()
            // punctuation keys
            .event(Key::Minus)
            .unwrap()
            .event(Key::Equal)
            .unwrap()
            .event(Key::LeftBrace)
            .unwrap()
            .event(Key::RightBrace)
            .unwrap()
            .event(Key::BackSlash)
            .unwrap()
            .event(Key::SemiColon)
            .unwrap()
            .event(Key::Apostrophe)
            .unwrap()
            .event(Key::Comma)
            .unwrap()
            .event(Key::Dot)
            .unwrap()
            .event(Key::Slash)
            .unwrap()
            // control
            .event(Key::Space)
            .unwrap()
            .event(Key::Tab)
            .unwrap()
            .event(Key::Enter)
            .unwrap()
            .event(Key::BackSpace)
            .unwrap()
            .event(Key::Esc)
            .unwrap()
            // modifiers
            .event(Key::LeftShift)
            .unwrap()
            .event(Key::RightShift)
            .unwrap()
            .event(Key::LeftControl)
            .unwrap()
            .event(Key::RightControl)
            .unwrap()
            .event(Key::LeftAlt)
            .unwrap()
            .event(Key::RightAlt)
            .unwrap()
            .event(Key::LeftMeta)
            .unwrap()
            .event(Key::RightMeta)
            .unwrap()
            // arrows
            .event(Key::Up)
            .unwrap()
            .event(Key::Down)
            .unwrap()
            .event(Key::Left)
            .unwrap()
            .event(Key::Right)
            .unwrap()
            .create()
            .unwrap();

        Self { device }
    }

    pub fn press(&mut self, key: Key) {
        self.device.press(&key).unwrap();
        self.device.release(&key).unwrap();
        thread::sleep(Duration::from_millis(40));
    }

    pub fn combo(&mut self, a: Key, b: Key) {
        self.device.press(&a).unwrap();
        self.device.press(&b).unwrap();

        self.device.release(&b).unwrap();
        self.device.release(&a).unwrap();

        thread::sleep(Duration::from_millis(80));
    }

    pub fn wait(&self, ms: u64) {
        thread::sleep(Duration::from_millis(ms));
    }

    pub fn type_char(&mut self, c: char) {
        match c {
            'a' => self.press(Key::A),
            'b' => self.press(Key::B),
            'c' => self.press(Key::C),
            'd' => self.press(Key::D),
            'e' => self.press(Key::E),
            'f' => self.press(Key::F),
            'g' => self.press(Key::G),
            'h' => self.press(Key::H),
            'i' => self.press(Key::I),
            'j' => self.press(Key::J),
            'k' => self.press(Key::K),
            'l' => self.press(Key::L),
            'm' => self.press(Key::M),
            'n' => self.press(Key::N),
            'o' => self.press(Key::O),
            'p' => self.press(Key::P),
            'q' => self.press(Key::Q),
            'r' => self.press(Key::R),
            's' => self.press(Key::S),
            't' => self.press(Key::T),
            'u' => self.press(Key::U),
            'v' => self.press(Key::V),
            'w' => self.press(Key::W),
            'x' => self.press(Key::X),
            'y' => self.press(Key::Y),
            'z' => self.press(Key::Z),

            ' ' => self.press(Key::Space),

            '\'' => self.press(Key::Apostrophe),

            '"' => {
                self.device.press(&Key::LeftShift).unwrap();
                self.press(Key::Apostrophe);
                self.device.release(&Key::LeftShift).unwrap();
            }

            _ => println!("Unsupported char: {}", c),
        }
    }

    pub fn type_string(&mut self, text: &str) {
        for c in text.chars() {
            self.type_char(c);
        }
    }
}

pub mod dialog;
pub mod title_screen;

pub struct TitleDict;

impl TitleDict {
    fn new() -> Self {
        TitleDict
    }

    fn encode(&self, ch: char) -> u8 {
        match ch {
            ' ' => 0x20,
            '0'..='9' => 0xD6 + (ch as u8 - b'0'),
            'A'..='Z' => 0xE0 + (ch as u8 - b'A'),
            'c' => 0xFA, // copyright
            _ => 0x20,   // for unsupported characters, fall back to space
        }
    }
}

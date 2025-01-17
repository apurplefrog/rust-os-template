#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

pub struct ColorCode {
    pub foreground: Color,
    pub background: Color,
}

impl ColorCode {
    fn as_u8(&self) -> u8 {
        ((self.background as u8) << 4) | (self.foreground as u8)
    }
}

#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    color_code: u8,
}

const BUFFER_WIDTH: usize = 25;
const BUFFER_HEIGHT: usize = 80;

pub struct ScreenBuffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    pub column_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut ScreenBuffer,
}

impl Writer {
    pub fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            other_char => {
                if self.column_position >= BUFFER_WIDTH {
                    self.newline();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: other_char as u8,
                    color_code: self.color_code.as_u8(),
                };

                self.column_position += 1;
            }
        }
    }

    pub fn newline(&self) {}

    pub fn write_string(&mut self, s: &str) {
        for c in s.chars() {
            match c as u8 {
                0x20..=0x7e | b'\n' => self.write_char(c),
                _ => self.write_char(176u8 as char),
            }
        }
    }
}

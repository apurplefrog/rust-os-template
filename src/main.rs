#![no_std]
#![no_main]

mod vga;
use vga::*;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode {
            foreground: Color::White,
            background: Color::Green,
        },
        buffer: unsafe { &mut *(0xb8000 as *mut ScreenBuffer) },
    };

    writer.write_string("Hello, world!");
    loop {}
}

#[panic_handler]
fn panic(_i: &PanicInfo) -> ! {
    loop {}
}

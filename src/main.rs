#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod boot;
mod io;
mod utils;

const PIXELS: &[u8] = include_bytes!("out.bin");

#[no_mangle]
pub extern "C" fn main() -> ! {
    io::uart::print("Bad Apple, but it's on the DTEK-V chip!\n");
    io::uart::print("Written in Rust, check it out: https://github.com/PumpedSardines/dtekv-bad-apple");
    io::uart::print("\n");

    io::vga::init();

    let mut global_index: u16 = 0;
    let mut rendered_frames = 0;

    for i in 0..(PIXELS.len() / 2) {
        let ptr = 2 * i;
        let a = PIXELS[ptr] as u16;
        let b = PIXELS[ptr + 1] as u16;

        let raw_index = b << 8 | a;
        let new = raw_index & 0x8000 != 0;
        let black = raw_index & 0x4000 != 0;
        let index = raw_index & 0x3FFF;

        if new {
            rendered_frames += 1;
            io::vga::swap_buffers();
            // Remove some MS due to all the writing code taking some time
            utils::delay(1000 / 24 - 20);
            global_index = 0;
        }

        for ci in 0..index {
            let x = (ci + global_index) % 160;
            let y = (ci + global_index) / 160;

            let x = (x as i32) * 2;
            let y = (y as i32) * 2;

            let color = if black {
                io::vga::Color::new(0, 0, 0)
            } else {
                io::vga::Color::new(7, 7, 3)
            };

            io::vga::set_pixel(x, y, color);
            io::vga::set_pixel(x + 1, y + 1, color);
            io::vga::set_pixel(x, y + 1, color);
            io::vga::set_pixel(x + 1, y, color);
        }

        global_index += index;
    }

    loop {}
}

#[no_mangle]
pub extern "C" fn handle_interrupt() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn handle_exception() -> ! {
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    io::uart::print("[PANIC]: ");
    let file = info.location().unwrap().file();
    let line = info.location().unwrap().line();

    match info.message().as_str() {
        Some(message) => io::uart::print(message),
        None => io::uart::print("no message"),
    }

    io::uart::print(" at ");
    io::uart::print(file);
    io::uart::printc(b'\n');

    loop {}
}

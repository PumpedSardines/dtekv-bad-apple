use crate::utils;

const JTAG_UART: *mut u32 = 0x04000040 as *mut u32;
const JTAG_CTRL: *mut u32 = 0x04000044 as *mut u32;

pub fn printc(c: u8) {
    unsafe {
        while ((*JTAG_CTRL) & 0xffff0000) == 0 {}
        *JTAG_UART = c as u32;
    }
}

pub fn print(s: &str) {
    for c in s.bytes() {
        printc(c);
    }
}

pub fn print_dec(num: u32) {
    let mut buffer = [0; 10];
    utils::num_to_dec_string(num, &mut buffer);
    for i in 0..buffer.len() {
        if buffer[i] == 0 {
            break;
        }
        printc(buffer[i]);
    }
}

pub fn print_hex(num: u32) {
    let mut buffer = [0; 10];
    utils::num_to_hex_string(num, &mut buffer);
    for i in 0..buffer.len() {
        if buffer[i] == 0 {
            break;
        }
        printc(buffer[i]);
    }
}

pub fn println(s: &str) {
    print(s);
    printc(b'\n');
}

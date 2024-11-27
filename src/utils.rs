use core::arch::asm;

pub fn delay(ms: u32) {
    let mut ms = ms * 15000;

    // Loop for ms * 15000 cycles
    unsafe {
        asm!(
            "addi {0}, {0}, -1
            bnez {0}, -0x4",
            inout(reg) ms
        )
    };
}

pub fn num_to_dec_string(mut num: u32, buffer: &mut [u8; 10]) -> usize {
    let mut i = 0;
    loop {
        buffer[i] = (num % 10 + 48) as u8;
        num /= 10;
        i += 1;
        if num == 0 {
            break;
        }
    }

    buffer[..i].reverse();
    i
}

pub fn num_to_hex_string(mut num: u32, buffer: &mut [u8; 10]) -> usize {
    let mut i = 0;
    loop {
        let digit = num % 16;
        buffer[i] = if digit < 10 {
            digit + 48
        } else {
            digit + 55
        } as u8;
        num /= 16;
        i += 1;
        if num == 0 {
            break;
        }
    }

    buffer[..i].reverse();
    i
}

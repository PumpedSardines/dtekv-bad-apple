pub const SCREEN_WIDTH: i32 = 320;
pub const SCREEN_HEIGHT: i32 = 240;
const FRONT_BUFFER_ADDR: *mut u8 = 0x08000000 as *mut u8;
const BACK_BUFFER_ADDR: *mut u8 = (0x08000000 + SCREEN_WIDTH * SCREEN_HEIGHT) as *mut u8;
const DMA_ADDR: *mut u32 = 0x04000100 as *mut u32;

static mut CURRENT_BACK_BUFFER: *mut u8 = core::ptr::null_mut();

/// Returns if the VGA can swap buffers
fn swapping() -> bool {
    unsafe {
        // Status address
        let addr = DMA_ADDR.add(3);
        core::ptr::read_volatile(addr) & 0x1 != 0
    }
}

pub fn swap_buffers() {
    while swapping() {}

    unsafe {
        // Swap buffers
        core::ptr::write_volatile(DMA_ADDR, 0x0);

        CURRENT_BACK_BUFFER = if CURRENT_BACK_BUFFER == BACK_BUFFER_ADDR {
            FRONT_BUFFER_ADDR
        } else {
            BACK_BUFFER_ADDR
        };
    }
}

pub fn init() {
    unsafe {
        *(DMA_ADDR.add(1)) = BACK_BUFFER_ADDR as u32;
        CURRENT_BACK_BUFFER = BACK_BUFFER_ADDR;
    };
}

#[derive(Clone, Copy)]
pub struct Color(u8);

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self(((r & 0x7) << 5) | ((g & 0x7) << 2) | (b & 0x3))
    }
}

pub fn set_pixel(x: i32, y: i32, color: Color) {
    if unsafe { CURRENT_BACK_BUFFER.is_null() } {
        panic!("VGA not initialized");
    }

    if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
        panic!("Pixel out of bounds");
    }

    if x < 0 || y < 0 {
        panic!("Pixel out of bounds");
    }

    unsafe {
        core::ptr::write_volatile(CURRENT_BACK_BUFFER.add((y * SCREEN_WIDTH + x).try_into().unwrap()), color.0);
    }
}

#![no_std] // disable the Rust standard library
#![no_main] // disable the Rust entry point

extern crate rlibc;

use core::panic::PanicInfo;

/// Automatically called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// System entry point.
/// Must be named _start by convention.
#[no_mangle] // output non-cryptic function name
pub extern "C" fn _start() -> ! {
    // Write "Hello World!" to the framebuffer.
    // By convention the framebuffer is at this address in RAM.
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in b"Hello World!".iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

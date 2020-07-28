#![no_std] // disable the Rust standard library
#![no_main] // disable the Rust entry point

extern crate rlibc;
mod vga_buffer;

use core::panic::PanicInfo;


/// Automatically called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// System entry point.
/// Must be named _start by convention.
#[no_mangle] // output non-cryptic function name
pub extern "C" fn _start() -> ! {
    println!("Hello world!");
    loop {}
}

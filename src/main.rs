#![no_std] // disable the Rust standard library
#![no_main] // disable the Rust entry point

use core::panic::PanicInfo;

/// Automatically called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// System entry point.
#[no_mangle] // output non-cryptic function name
pub extern "C" fn _start() -> ! {
    loop {}
}
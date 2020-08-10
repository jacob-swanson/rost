#![no_std] // disable the Rust standard library
#![no_main] // disable the Rust entry point
#![feature(custom_test_frameworks)]
#![test_runner(rost::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rost::println;

/// System entry point.
/// Must be named _start by convention.
#[no_mangle] // output non-cryptic function name
pub extern "C" fn _start() -> ! {
    println!("Hello world!");

    rost::init();

    #[cfg(test)]
    test_main();
    loop {}
}


/// Called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rost::test_panic_handler(info);
}

#[test_case]
fn trivial_test() {
    assert_eq!(1, 1);
}

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use rost::{exit_qemu, QemuExitCode, serial_print, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    rost::gdt::init();
    init_test_idt();

    // trigger stack overflow
    stack_overflow();

    panic!("Execution continued after stack overflow")
}

#[allow(unconditional_recursion)] // silence warning
fn stack_overflow() {
    stack_overflow(); // push a new return address
    // prevent tail recursion optimization which would convert this to a simple loop leading to no
    // additions to the stack
    volatile::Volatile::new(0)
        .read();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rost::test_panic_handler(info);
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(rost::gdt::DOUBLE_FAULT_IST_INDEX);
        };
        idt
    };
}

/// Pass test when double fault is handled.
extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

pub fn init_test_idt() {
    TEST_IDT.load();
}
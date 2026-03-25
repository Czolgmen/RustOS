#![no_std] // No standard libraries
#![no_main] // No main function
#![feature(custom_test_frameworks)] // Custom test frameworks
#![test_runner(crate::test_runner)] // Which function runs tests
#![reexport_test_harness_main = "test_main"] // No clash with main functions

mod serial;
mod vga_buffer;
#[allow(unused_imports)]
use core::arch::asm;
use core::panic::PanicInfo; // Allow for inline assembly

#[cfg(not(test))] // Used when not a test setup
#[panic_handler] // Function called when rust panic is triggered -> kernel panic
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)] // Used when in a test setup
#[panic_handler] // Function called when rust panic is triggered -> kernel panic
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[FAILED]");
    serial_println!("Error: {}", _info);

    exit_qemu(QemuExitCodes::ExitFailure);
    loop {}
}

#[unsafe(no_mangle)] // Do not mangle the name of the next function
pub extern "C" fn _start() -> ! {
    // Entry point to the os
    println! {"Hello World!"};

    #[cfg(test)]
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} Tests!", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCodes::ExitSuccess);
}

#[test_case]
#[allow(clippy::eq_op)]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCodes {
    ExitSuccess = 0x10,
    ExitFailure = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCodes) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

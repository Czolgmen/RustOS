#![no_std] // No standard libraries
#![no_main] // No main function
#![feature(custom_test_frameworks)] // Custom test frameworks
#![test_runner(crate::test_runner)] // Which function runs tests
#![reexport_test_harness_main = "test_main"] // No clash with main functions

mod vga_buffer;
#[allow(unused_imports)]
use core::arch::asm;
use core::panic::PanicInfo; // Allow for inline assembly

#[panic_handler] // Function called when rust panic is triggered -> kernel panic
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[unsafe(no_mangle)] // Do not mangle the name of the next function
pub extern "C" fn _start() -> ! {
    // Entry point to the os
    println! {"Hello World!"};

    #[cfg(test)]
    test_main();

    //panic!("Panigga");
    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} Tests!", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCodes::ExitSuccess);
}

#[test_case]
#[allow(clippy::eq_op)]
fn trivial_assertion() {
    print!("Trivial assert ...");
    assert_eq!(1, 1);
    print!("[ok].\n");
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

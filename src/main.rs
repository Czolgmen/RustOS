#![no_std] // No standard libraries
#![no_main] // No main function
#![feature(custom_test_frameworks)] // Custom test frameworks
#![test_runner(rust_os::test_runner)] // Which function runs tests
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
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(_info);
}

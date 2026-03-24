#![no_std] // No standard libraries
#![no_main] // No main function

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

    panic!("Panika");

    loop {}
}

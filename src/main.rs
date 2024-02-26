#![no_std]

#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(marsos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;
mod serial;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello {}", "MarsOS");

    marsos::init(); // new

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    marsos::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    marsos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    marsos::test_panic_handler(info)
}
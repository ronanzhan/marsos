#![no_std]

#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(marsos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use x86_64::structures::paging::{Page, Translate};
use x86_64::VirtAddr;

use marsos::memory;

mod vga_buffer;
mod serial;


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello {}\n", "MarsOS");

    marsos::init();

    #[cfg(test)]
    test_main();

    println!("MarsOS booting successfully!");

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
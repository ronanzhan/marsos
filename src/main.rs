#![no_std]

#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(marsos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use x86_64::structures::paging::Translate;
use x86_64::VirtAddr;
use marsos::{allocator, memory};
use marsos::memory::BootInfoFrameAllocator;

mod vga_buffer;
mod serial;


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello {}\n", "MarsOS");
    marsos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // new
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let heap_value = Box::new(1);
    println!("heap_value at {:p}", heap_value);

    let heap_value2 = Box::new(1);
    println!("heap_value at {:p}", heap_value2);


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
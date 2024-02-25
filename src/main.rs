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
    println!("123456789022345678903234567890423456789052345678906234567890");

    marsos::init(); // new

    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3(); // new

    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // };

    fn stack_overflow() {
        stack_overflow(); // 每一次递归都会将返回地址入栈
    }

    // 触发 stack overflow
    stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    marsos::test_panic_handler(info)
}
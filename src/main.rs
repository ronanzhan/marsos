// #![deny(missing_docs)]
// #![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

#[macro_use]
mod console;
mod config;
mod lang_items;
mod loader;
mod sbi;
mod sync;
pub mod syscall;
pub mod task;
pub mod trap;

mod logging;
mod timer;

#[path = "boards/qemu.rs"]
mod board;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

#[no_mangle]
pub fn marsos_entry() -> ! {
    println!("[kernel] MarsOS is booting...");
    logging::init();
    print_section_info();
    clear_bss();
    println!("[kernel] Hello MarsOS");
    trap::init();
    loader::load_apps();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}

pub fn print_section_info() {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();

        fn __restore();
    }

    println!("[kernel] text:\t {:x}..{:x}", stext as usize, etext as usize);
    println!("[kernel] rodata: {:x}..{:x}", srodata as usize, erodata as usize);
    println!("[kernel] data:\t {:x}..{:x}", sdata as usize, edata as usize);
    println!("[kernel] bss:\t {:x}..{:x}", sbss as usize, ebss as usize);
    println!("[kernel] __restore: {}", __restore as usize);
}
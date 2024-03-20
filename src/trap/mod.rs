use core::arch::global_asm;

use riscv::register::{scause, sie, stval, stvec};
use riscv::register::mtvec::TrapMode;
use riscv::register::scause::{Exception, Interrupt, Trap};

pub use context::TrapContext;

use crate::syscall::syscall;
use crate::task::suspend_current_and_run_next;
use crate::timer::set_next_trigger;

mod context;

global_asm!(include_str!("trap.S"));

pub fn init() {
    println!("[kernel] Start initializing Trap System...");
    extern "C" {
        fn __alltraps();
    }

    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
    println!("[kernel] initializing Trap System succeeded...");
}


#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            panic!("[kernel] Cannot continue!");
            // run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            panic!("[kernel] Cannot continue!");
            // run_next_app();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}
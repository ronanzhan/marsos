#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct TaskContext {
    // 返回地址
    ra: usize,
    sp: usize,
    //保存12个被调用者保存的寄存器，也就是 s0-s11，这部分寄存器是需要被调用者来负责保存的
    s: [usize; 12],
}

impl TaskContext {
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
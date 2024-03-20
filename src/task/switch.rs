use core::arch::global_asm;
use crate::task::TaskContext;



global_asm!(include_str!("__switch.S"));


extern "C" {
    /**
     * 主要工作是切换应用的内核栈
     */
    pub fn __switch(current_task_ctx_ptr: *mut TaskContext, next_task_ctx_ptr: *const TaskContext);
}
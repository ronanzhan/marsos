use lazy_static::*;

pub use context::TaskContext;

use crate::config::MAX_APP_NUM;
use crate::loader;
use crate::sbi::shutdown;
use crate::sync::UPSafeCell;
use crate::task::switch::__switch;
use crate::task::task::{TaskControlBlock, TaskStatus};
use crate::task::task::TaskStatus::Running;

mod task;
mod switch;
mod context;

lazy_static! {
    static ref TASK_MANAGER: TaskManager = {
        let num_app = loader::get_num_app();
        let mut tasks = [
            TaskControlBlock {
                task_ctx: TaskContext::zero_init(),
                task_status: TaskStatus::UnInit
            };
            MAX_APP_NUM
        ];
        for i in 0..num_app {
            tasks[i].task_ctx = TaskContext::goto_restore(loader::init_app_context(i));
            tasks[i].task_status = TaskStatus::Ready;

        }
        TaskManager{
            num_app,
            inner: unsafe { UPSafeCell::new(TaskManagerInner {
                tasks,
                current_task: 0,
            })},
        }
    };
}

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}


struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

impl TaskManager {
    fn run_first_task(&self) {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = Running;
        let next_task_ctx_ptr = &task0.task_ctx as *const TaskContext;
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        unsafe {
            __switch(
                &mut _unused as *mut TaskContext,
                next_task_ctx_ptr,
            )
        }
        panic!("unreachable in run_first_task!");
    }
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    pub fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;

            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_ctx_ptr = &mut inner.tasks[current].task_ctx as *mut TaskContext;
            let next_task_ctx_ptr = &mut inner.tasks[next].task_ctx as *mut TaskContext;
            drop(inner);

            unsafe {
                //switch完就会执行__restore，因为切换了栈，A->B，就会加载应用B的运行时数据，
                //包括ra，这个ra是__switch的返回地址，初始化的时候是__restroe（goto_restore），也就是说应用第一次执行后会跳到__restore执行
                // 包括sepc（已经修改为B的下一条指令）等，就会执行B的指令
                __switch(
                    current_task_ctx_ptr,
                    next_task_ctx_ptr,
                );
            }
        } else {
            println!("All applications completed!");
            shutdown(false);
        }
    }
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| {
                inner.tasks[*id].task_status == TaskStatus::Ready
            })
    }
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    TASK_MANAGER.run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    TASK_MANAGER.run_next_task();
}

pub fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

pub fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}


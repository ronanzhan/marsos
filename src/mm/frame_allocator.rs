use alloc::vec::Vec;
use lazy_static::lazy_static;
use crate::config::MEMORY_END;
use crate::mm::address::{PhysAddr, PhysPageNum};
use crate::sync::UPSafeCell;

type FrameAllocatorImpl = StackFrameAllocator;

lazy_static! {
    pub static ref FRAME_ALLOCATOR : UPSafeCell<FrameAllocatorImpl> = unsafe {
        UPSafeCell::new(FrameAllocatorImpl::new())
    };
}

pub fn init_frame_allocator() {
    extern "C" {
        fn ekernel();
    }

    FRAME_ALLOCATOR.exclusive_access().init(PhysAddr::from(ekernel as usize).ceil(), PhysAddr::from(MEMORY_END).floor());
}


trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;

    fn dealloc(&mut self, ppn: PhysPageNum);
}

pub struct StackFrameAllocator {
    ///空闲内存的起始物理页号
    current: usize,
    ///空闲内存的结束物理页号，不包含
    end: usize,
    ///用来存放回收的物理页号
    recycled: Vec<usize>,
}

impl StackFrameAllocator {
    pub fn init(&mut self, current: PhysPageNum, end: PhysPageNum) {
        self.current = current.0;
        self.end = end.0;
    }
}

impl FrameAllocator for StackFrameAllocator {
    ///新建一个物理页帧分配器
    /// ```
    /// StackFrameAllocator::new()
    /// ```
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }

    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            Some(ppn.into())
        } else {
            if self.current == self.end {
                None
            } else {
                self.current += 1;
                Some((self.current - 1).into())
            }
        }
    }

    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;

        if ppn >= self.current
            || self.recycled.iter()
            .find(|&v| { *v == ppn })
            .is_some() {
            panic!("Frame ppn={:#x} has not been allocated!", ppn);
        }

        self.recycled.push(ppn);
    }
}


pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}

impl FrameTracker {
    /// 初始化物理页号起开始的字节数组，将其作为一个页帧，总共4k，并将其中所有字节初始化为0
    pub fn new(ppn: PhysPageNum) -> Self {
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array {
            *i = 0;
        }
        Self { ppn }
    }
}

pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR.exclusive_access().alloc()
        .map(|ppn| FrameTracker::new(ppn))
}

fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.exclusive_access().dealloc(ppn);
}
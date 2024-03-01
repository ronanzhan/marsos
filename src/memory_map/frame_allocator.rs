use alloc::vec::Vec;

trait FrameAllocator {
    fn new() -> Self;
    fn allocate(&mut self) -> Option<PhysicalPageNumber>;
    fn deallocate(&mut self, physical_page_number: PhysicalPageNumber);
}


pub struct StackFrameAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>,
}

impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }

    fn allocate(&mut self) -> Option<PhysicalPageNumber> {
        todo!()
    }

    fn deallocate(&mut self, physical_page_number: PhysicalPageNumber) {
        todo!()
    }
}

struct PhysicalPageNumber;

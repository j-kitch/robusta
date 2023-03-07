use crate::heap::Heap;
use crate::method_area::MethodArea;
use crate::native::NativeMethods;

pub struct Runtime {
    pub heap: Box<Heap>,
    pub method_area: Box<MethodArea>,
    pub native: Box<NativeMethods>,
}

unsafe impl Send for Runtime {}

impl Runtime {
    pub fn new() -> Self {
        let heap = Box::new(Heap::new());
        let method_area = Box::new(MethodArea::new(heap.as_ref() as *const Heap));
        Runtime { heap, method_area, native: Box::new(NativeMethods::new()) }
    }
}
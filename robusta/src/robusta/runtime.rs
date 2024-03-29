use std::sync::{Arc, RwLock};

use crate::heap::Heap;
use crate::method_area::MethodArea;
use crate::native::NativeMethods;
use crate::thread::Thread;

pub struct Runtime {
    pub heap: Box<Heap>,
    pub method_area: Box<MethodArea>,
    pub native: Box<NativeMethods>,
    pub threads2: RwLock<Vec<Arc<Thread>>>,
}

unsafe impl Send for Runtime {}

impl Runtime {
    pub fn new() -> Arc<Self> {
        let heap = Box::new(Heap::new());
        let method_area = Box::new(MethodArea::new(heap.as_ref() as *const Heap));
        let rt = Arc::new(Runtime {
            heap,
            method_area,
            native: Box::new(NativeMethods::new()),
            threads2: RwLock::new(Vec::new()),
        });
        rt.heap.allocator.set_rt(rt.clone());
        rt
    }

    pub fn clear(&self) {
        self.method_area.clear();
    }
}
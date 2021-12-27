use crate::class_loader::ClassLoader;
use crate::heap::Heap;

pub struct Runtime {
    pub class_loader: ClassLoader,
    pub heap: Heap,
}

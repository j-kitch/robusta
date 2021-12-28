use crate::class_loader::ClassLoader;
use crate::heap::Heap;
use crate::native::NativeMethods;

pub struct Runtime {
    pub class_loader: ClassLoader,
    pub heap: Heap,
    pub native: NativeMethods,
}

//! Module `runtime` defines the common runtime areas that are shared across all threads in a
//! single instance of a running JVM.

use std::path::PathBuf;
use std::sync::Arc;

pub use const_pool::Const;
pub use const_pool::ConstPool;
pub use method_area::{Method, MethodArea};

use crate::loader::ClassFileLoader;
use crate::native::NativeMethods;
use crate::runtime::heap::Heap;

pub mod method_area;
pub mod const_pool;
pub mod heap;
pub mod heap3;

/// The runtime of a Java Virtual Machine consists of the method area, the runtime constant pools
/// and the heap.
pub struct Runtime {
    pub method_area: Arc<MethodArea>,
    pub heap: Arc<Heap>,
    pub native: NativeMethods,
    pub loader: ClassFileLoader,
}

impl Runtime {
    pub fn new() -> Arc<Self> {
        Arc::new(Runtime {
            method_area: MethodArea::new(),
            heap: Heap::new(),
            native: NativeMethods::new(),
            loader: ClassFileLoader::new(vec![
                PathBuf::from("./classes"),
                PathBuf::from("./classes/EmptyMain.jar")
            ]),
        })
    }
}


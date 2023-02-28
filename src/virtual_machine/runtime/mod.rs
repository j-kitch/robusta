//! Module `runtime` defines the common runtime areas that are shared across all threads in a
//! single instance of a running JVM.

use std::sync::Arc;

pub use const_pool::ConstPool;
pub use method_area::{Method, MethodArea};

mod method_area;
mod const_pool;

/// The runtime of a Java Virtual Machine consists of the method area, the runtime constant pools
/// and the heap.
pub struct Runtime {
    pub method_area: Arc<MethodArea>,
}

impl Runtime {
    pub fn new() -> Arc<Self> {
        Arc::new(Runtime {
            method_area: MethodArea::new()
        })
    }
}


//! This module defines the Robusta implementation of a Java Virtual Machine, as defined
//! in the [specification](https://docs.oracle.com/javase/specs/jvms/se8/html/index.html).

use std::sync::Arc;

use crate::java::MethodType;
use crate::virtual_machine::runtime::Runtime;
use crate::virtual_machine::thread::Thread;

mod runtime;
mod thread;

/// A single instance of a Java Virtual Machine, capable of running a Java program.
pub struct VirtualMachine {
    runtime: Arc<Runtime>,
    main_thread: Thread,
}

impl VirtualMachine {
    pub fn new(main_class: &str) -> Self {
        let runtime = Runtime::new();

        runtime.method_area.insert(runtime.heap.clone(), main_class);

        let pool = runtime.method_area.find_const_pool(main_class);
        let method = runtime.method_area.find_method(main_class, "main", &MethodType::from_descriptor("([Ljava/lang/String;)V").unwrap());

        let main_thread = Thread::new(runtime.clone(), pool, method);

        VirtualMachine { runtime, main_thread }
    }

    pub fn start(&mut self) {
        self.main_thread.run()
    }
}
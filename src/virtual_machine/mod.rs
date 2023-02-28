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

        let main_class = runtime.method_area.insert(main_class);
        let main_method = main_class.methods.iter()
            .find(|m| m.name.eq("main") && m.descriptor.eq(&MethodType::from_descriptor("([Ljava/lang/String;)V").unwrap()))
            .unwrap()
            .clone();

        let main_thread = Thread::new(runtime.clone(), main_class.const_pool, main_method);

        VirtualMachine { runtime, main_thread }
    }

    pub fn start(&mut self) {
        self.main_thread.run()
    }
}
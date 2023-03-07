//! This module defines the Robusta implementation of a Java Virtual Machine, as defined
//! in the [specification](https://docs.oracle.com/javase/specs/jvms/se8/html/index.html).

extern crate core;

use std::sync::Arc;

use crate::java::MethodType;
use crate::runtime::Runtime;
use crate::thread::Thread;

pub mod java;
pub mod class_file;
pub mod collection;
pub mod native;
pub mod runtime;
pub mod thread;
mod instruction;
mod loader;
mod method_area;
mod heap;

/// A single instance of a Java Virtual Machine, capable of running a Java program.
#[allow(dead_code)]
pub struct VirtualMachine {
    runtime: Arc<Runtime>,
    main_thread: Thread,
}

impl VirtualMachine {
    pub fn new(main_class: &str) -> Self {
        let runtime = Runtime::new();

        for class in [
            "java.lang.Object",
            "java.lang.Class",
            "java.lang.String"
        ] {
            runtime.method_area.insert(runtime.clone(), class);
        }

        runtime.method_area.insert(runtime.clone(), main_class);

        let pool = runtime.method_area.find_const_pool(main_class);
        let method = runtime.method_area.find_method(main_class, "main", &MethodType::from_descriptor("([Ljava/lang/String;)V").unwrap());

        let main_thread = Thread::new(runtime.clone(), main_class.to_string(), pool, method);

        VirtualMachine { runtime, main_thread }
    }

    pub fn start(&mut self) {
        self.main_thread.run()
    }
}

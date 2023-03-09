//! This module defines the Robusta implementation of a Java Virtual Machine, as defined
//! in the [specification](https://docs.oracle.com/javase/specs/jvms/se8/html/index.html).

extern crate core;

use std::env::args;
use std::sync::Arc;

use tracing::debug;
use tracing::metadata::LevelFilter;
use tracing::subscriber::set_global_default;
use tracing_subscriber::{EnvFilter, fmt};
use tracing_subscriber::fmt::format::FmtSpan;

use crate::java::MethodType;
use crate::method_area::const_pool::{ConstPool, MethodKey};
use crate::method_area::Method;
use crate::runtime::Runtime;
use crate::thread::Thread;

pub mod java;
pub mod class_file;
pub mod collection;
pub mod native;
pub mod thread;
mod instruction;
mod loader;
mod method_area;
mod heap;
mod runtime;
mod log;

/// A single instance of a Java Virtual Machine, capable of running a Java program.
#[allow(dead_code)]
pub struct VirtualMachine {
    runtime: Arc<Runtime>,
    main_thread: Thread,
}

impl VirtualMachine {
    pub fn new(main_class: &str) -> Self {
        let is_debug = args().any(|arg| arg.eq("-d"));
        let is_trace = args().any(|arg| arg.eq("-t"));

        let mut filter = EnvFilter::builder()
            .with_default_directive(LevelFilter::OFF.into());

        if is_debug {
            filter = filter.with_default_directive(LevelFilter::DEBUG.into());
        }
        if is_trace {
            filter = filter.with_default_directive(LevelFilter::TRACE.into());
        }

        let filter: EnvFilter = filter.parse("").unwrap();

        let subscriber = fmt()
            .without_time()
            .with_span_events(FmtSpan::FULL)
            .with_target(true)
            .with_level(true)
            .with_thread_names(true)
            .with_env_filter(filter)
            .finish();
        set_global_default(subscriber).unwrap();

        debug!(target: log::JVM, "Starting Robusta");

        let runtime = Arc::new(Runtime::new());

        // for class in [
        //     "java.lang.Object",
        //     "java.lang.Class",
        //     "java.lang.String"
        // ] {
        //     runtime.method_area.load_class(class);
        // }

        let main_class = runtime.method_area.load_class(main_class);
        let method = main_class.find_method(&MethodKey {
            class: main_class.name.clone(),
            name: "main".to_string(),
            descriptor: MethodType::from_descriptor("([Ljava/lang/String;)V").unwrap(),
        }).unwrap();

        let main_thread = Thread::new(
            None,
            runtime.clone(),
            main_class.name.clone(),
            &main_class.const_pool as *const ConstPool,
            method as *const Method);

        VirtualMachine { runtime, main_thread }
    }

    pub fn start(&mut self) {
        self.main_thread.run()
    }
}

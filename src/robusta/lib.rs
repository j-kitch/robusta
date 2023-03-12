//! This module defines the Robusta implementation of a Java Virtual Machine, as defined
//! in the [specification](https://docs.oracle.com/javase/specs/jvms/se8/html/index.html).

extern crate core;

use std::env::args;
use std::sync::Arc;

use tracing::{debug, trace};
use tracing::metadata::LevelFilter;
use tracing::subscriber::set_global_default;
use tracing_subscriber::{EnvFilter, fmt};
use tracing_subscriber::fmt::format::FmtSpan;

use crate::java::MethodType;
use crate::method_area::const_pool::{ConstPool, MethodKey};
use crate::method_area::Method;
use crate::runtime::Runtime;
use crate::thread::{Frame, LocalVars, OperandStack, Thread};

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
mod shim;

/// A single instance of a Java Virtual Machine, capable of running a Java program.
#[allow(dead_code)]
pub struct VirtualMachine {
    runtime: Arc<Runtime>,
    main_thread: Arc<Thread>,
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

        // Required Initialization
        let main_thread_ref = {
            let create_thread_class = runtime.method_area.insert_gen_class(shim::create_main_thread());
            let class_ref = unsafe { create_thread_class.as_ref().unwrap() };
            let method = &class_ref.methods[0];

            let jvm_init_thread = Thread::new("<jvmInit>".to_string(), None, runtime.clone(),
                class_ref.name.clone(), &class_ref.const_pool as *const ConstPool,
                method as *const Method);

            let jvm_init_t = jvm_init_thread.as_mut();

            jvm_init_t.stack.insert(0, Frame {
                class: "<result>".to_string(),
                const_pool: 0 as *const ConstPool,
                method: 0 as *const Method,
                operand_stack: OperandStack::new(),
                local_vars: LocalVars::new(),
                pc: 0,
            });

            while jvm_init_t.stack.len() > 1 {
                jvm_init_t.next();
                trace!("here 1");
            }

            jvm_init_t.stack.last_mut().unwrap().operand_stack.pop().reference()
        };

        let main_class = runtime.method_area.load_class(main_class);
        let method = main_class.find_method(&MethodKey {
            class: main_class.name.clone(),
            name: "main".to_string(),
            descriptor: MethodType::from_descriptor("([Ljava/lang/String;)V").unwrap(),
        }).unwrap();

        let main_thread = Thread::new(
            "main".to_string(),
            Some(main_thread_ref),
            runtime.clone(),
            main_class.name.clone(),
            &main_class.const_pool as *const ConstPool,
            method as *const Method);

        VirtualMachine { runtime, main_thread }
    }

    pub fn start(&mut self) {
        self.main_thread.as_mut().run()
    }
}

// #![feature(test)]
//! This module defines the Robusta implementation of a Java Virtual Machine, as defined
//! in the [specification](https://docs.oracle.com/javase/specs/jvms/se8/html/index.html).

// extern crate test;

extern crate core;

use std::env::args;
use std::sync::Arc;

use tracing::debug;
use tracing::metadata::LevelFilter;
use tracing::subscriber::set_global_default;
use tracing_subscriber::{EnvFilter, fmt};
use tracing_subscriber::fmt::format::FmtSpan;

use crate::java::{Int, MethodType, Reference, Value};
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
pub mod loader;
pub mod method_area;
mod heap;
pub mod runtime;
mod log;
mod shim;

/// A single instance of a Java Virtual Machine, capable of running a Java program.
#[allow(dead_code)]
pub struct VirtualMachine {
    runtime: Arc<Runtime>,
    main_thread: Arc<Thread>,
}

impl VirtualMachine {
    pub fn new() -> Self {
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
            .with_ansi(false)
            .with_span_events(FmtSpan::FULL)
            .with_target(true)
            .with_level(true)
            .with_thread_names(true)
            .with_env_filter(filter)
            .finish();
        set_global_default(subscriber).unwrap();

        debug!(target: log::JVM, "Starting Robusta");

        let runtime = Runtime::new();
        runtime.method_area.load_class("sun.misc.Launcher");

        // Required Initialization
        {
            let create_thread_class = runtime.method_area.insert_gen_class(shim::create_main_thread());
            let class_ref = unsafe { create_thread_class.as_ref().unwrap() };
            let method = &class_ref.methods[0];

            let jvm_init_thread = Thread::new("<jvmInit>".to_string(), None, runtime.clone(),
                                              class_ref.name.clone(), &class_ref.const_pool as *const ConstPool,
                                              method as *const Method, vec![]);

            let jvm_init_t = jvm_init_thread.as_mut();

            while jvm_init_t.stack.len() > 0 {
                jvm_init_t.next();
            }

            // Always safe.
            jvm_init_t.safe.enter();

            // jvm_init_t.stack.last_mut().unwrap().operand_stack.pop().reference()
        };

        let string_args: Vec<Reference> = args()
            .skip(1)
            .skip_while(|arg| arg.starts_with('-'))
            .skip(1) // main class
            .map(|arg| runtime.method_area.load_string(&arg))
            .collect();

        let args_arr_ref = runtime.heap.new_array(
            runtime.method_area.load_outer_class("java.lang.String"),
            Int(string_args.len() as i32));
        let args_arr = runtime.heap.get_array(args_arr_ref);
        for (idx, arg) in string_args.iter().enumerate() {
            args_arr.set_element(Int(idx as i32), Value::Reference(arg.clone()));
        }

        let main_class = args()
            .skip(1)
            .skip_while(|arg| arg.starts_with('-'))
            .next().unwrap();

        let main_class = runtime.method_area.load_class(&main_class);
        let method = main_class.find_method(&MethodKey {
            class: main_class.name.clone(),
            name: "main".to_string(),
            descriptor: MethodType::from_descriptor("([Ljava/lang/String;)V").unwrap(),
        }).unwrap();


        let main_thread = Thread::new(
            "main".to_string(),
            None,
            runtime.clone(),
            main_class.name.clone(),
            &main_class.const_pool as *const ConstPool,
            method as *const Method,
            vec![Value::Reference(args_arr_ref)]);

        VirtualMachine { runtime, main_thread }
    }

    pub fn start(&mut self) {
        self.main_thread.as_mut().run()
    }
}

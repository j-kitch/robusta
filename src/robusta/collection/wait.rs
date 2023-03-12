use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

use tracing::info;

use crate::java::Reference;
use crate::log;
use crate::runtime::Runtime;

pub struct ThreadWait {
    runtime: Arc<Runtime>,
    thread_ref: Mutex<Reference>,
    cond_var: Condvar,
}

impl ThreadWait {
    pub fn new(runtime: Arc<Runtime>, reference: Reference) -> Self {
        ThreadWait {
            runtime,
            thread_ref: Mutex::new(reference),
            cond_var: Condvar::new(),
        }
    }

    pub fn end(&self) {
        self.cond_var.notify_all();
    }

    pub fn wait_gc(&self) {
        self.cond_var.notify_all();
    }

    pub fn join(&self) {
        let thread_ref = self.thread_ref.lock().unwrap();
        self.cond_var.wait_while(thread_ref, |reference| {
            self.runtime.heap.get_thread_alive(*reference)
        }).unwrap().0;
    }

    pub fn join_millis(&self, millis: i64) {
        let thread_ref = self.thread_ref.lock().unwrap();
        self.cond_var.wait_timeout_while(thread_ref, Duration::from_millis(millis as u64), |reference| {
            self.runtime.heap.get_thread_alive(*reference)
        }).unwrap().0.0;
    }
}
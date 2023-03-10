use std::cell::Ref;
use std::collections::HashSet;
use std::sync::{Condvar, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::heap::garbage_collector::thread_roots;
use crate::java::Reference;
use crate::thread::Thread;

pub struct SafePoint {
    /// The GC has requested for the JVM to wait at a safe point.
    requested: (Mutex<bool>, Condvar),
    /// The number of threads that are registered with the JVM.
    num_threads: AtomicUsize,
    /// The number of threads that are waiting for GC to occur, each thread
    /// has sent all it's roots to be consumed.
    waiting_threads: (Mutex<Vec<HashSet<Reference>>>, Condvar),
}

impl SafePoint {
    pub fn new() -> Self {
        SafePoint {
            requested: (Mutex::new(false), Condvar::new()),
            num_threads: AtomicUsize::new(0),
            waiting_threads: (Mutex::new(Vec::new()), Condvar::new()),
        }
    }

    pub fn register_thread(&self) {
        self.num_threads.fetch_add(1, Ordering::SeqCst);
    }

    pub fn remove_thread(&self) {
        self.num_threads.fetch_sub(1, Ordering::SeqCst);
    }

    pub fn start_gc(&self, thread: &Thread) {
        {
            let mut requested = self.requested.0.lock().unwrap();
            *requested = true;
        }
        let num_threads = self.num_threads.load(Ordering::SeqCst);

        let roots = thread_roots(thread);
        let mut waiting_threads = self.waiting_threads.0.lock().unwrap();
        waiting_threads.push(roots);
        let condvar = &self.waiting_threads.1;

        let _unused = condvar.wait_while(waiting_threads, |n| n.len() < num_threads).unwrap();
    }

    pub fn end_gc(&self) {
        {
            let mut requested = self.requested.0.lock().unwrap();
            *requested = false;
        }
        self.requested.1.notify_all();
    }

    pub fn enter_safe_point(&self, thread: &Thread) {
        let is_safe_point = {
            let lock = self.requested.0.lock().unwrap();
            *lock
        };

        if is_safe_point {
            {
                let roots = thread_roots(thread);
                let mut waiting = self.waiting_threads.0.lock().unwrap();
                waiting.push(roots);
            }
            let lock = self.requested.0.lock().unwrap();
            let _unused = self.requested.1.wait_while(lock, |is_safe_point| *is_safe_point)
                .unwrap();
        }
    }

    pub fn consume_roots(&self) -> HashSet<Reference> {
        let mut waiting = self.waiting_threads.0.lock().unwrap();
        waiting.drain(0..).flat_map(|set| set.into_iter()).collect()
    }
}
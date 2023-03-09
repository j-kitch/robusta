use std::sync::{Condvar, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub struct SafePoint {
    /// The GC has requested for the JVM to wait at a safe point.
    requested: (Mutex<bool>, Condvar),
    /// The number of threads that are registered with the JVM.
    num_threads: AtomicUsize,
    /// The number of threads that are waiting for GC to occur.
    waiting_threads: (Mutex<usize>, Condvar),
}

impl SafePoint {
    pub fn new() -> Self {
        SafePoint {
            requested: (Mutex::new(false), Condvar::new()),
            num_threads: AtomicUsize::new(0),
            waiting_threads: (Mutex::new(0), Condvar::new()),
        }
    }

    pub fn register_thread(&self) {
        self.num_threads.fetch_add(1, Ordering::SeqCst);
    }

    pub fn remove_thread(&self) {
        self.num_threads.fetch_sub(1, Ordering::SeqCst);
    }

    pub fn start_gc(&self) {
        {
            let mut requested = self.requested.0.lock().unwrap();
            *requested = true;
        }
        let num_threads = self.num_threads.load(Ordering::SeqCst);

        let mut waiting_threads = self.waiting_threads.0.lock().unwrap();
        *waiting_threads += 1;
        let condvar = &self.waiting_threads.1;

        let _unused = condvar.wait_while(waiting_threads, |n| *n < num_threads).unwrap();
    }

    pub fn end_gc(&self) {
        {
            let mut requested = self.requested.0.lock().unwrap();
            *requested = false;
        }
        self.requested.1.notify_all();
    }

    pub fn enter_safe_point(&self) {
        let is_safe_point = {
            let lock = self.requested.0.lock().unwrap();
            *lock
        };

        if is_safe_point {
            {
                let mut waiting = self.waiting_threads.0.lock().unwrap();
                *waiting += 1;
            }
            let lock = self.requested.0.lock().unwrap();
            let _unused = self.requested.1.wait_while(lock, |is_safe_point| *is_safe_point)
                .unwrap();
        }
    }
}
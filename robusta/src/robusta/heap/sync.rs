use std::sync::{Arc, RwLock};
use std::thread::{current, park, park_timeout, Thread};
use std::time::Duration;

use parking_lot::{RawMutex, RawThreadId};
use parking_lot::lock_api::{ArcReentrantMutexGuard, ReentrantMutex};

pub struct ObjectLock {
    mutex: Arc<ReentrantMutex<RawMutex, RawThreadId, ()>>,
    /// The wait set for this object lock, a map of the thread IDs to the number of reentries.
    waiting: Arc<RwLock<Vec<Thread>>>,
}

impl ObjectLock {
    pub fn new() -> Self {
        ObjectLock {
            mutex: Arc::new(ReentrantMutex::new(())),
            waiting: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn lock(&self) -> Synchronized {
        let guard = self.mutex.lock_arc();
        Synchronized {
            reentry: 1,
            _guard: guard,
        }
    }

    /// Blocking!
    /// - We don't seem to need to pass re-entry all the way through here?
    /// Only the thread handle actually seems to need to exist here.
    pub fn wait(&self, duration: Option<Duration>) {
        {
            let mut waiting = self.waiting.write().unwrap();
            waiting.push(current());
        }

        // Blocking wait here.
        if let Some(duration) = duration {
            park_timeout(duration);
        } else {
            park();
        }
    }

    pub fn notify(&self) {
        let mut waiting = self.waiting.write().unwrap();
        if let Some(notified) = waiting.pop() {
            notified.unpark();
        }
    }

    pub fn notify_all(&self) {
        let mut waiting = self.waiting.write().unwrap();
        for notified in waiting.iter() {
            notified.unpark();
        }
        waiting.clear();
    }

    pub fn move_me(&self) -> Self {
        ObjectLock {
            mutex: self.mutex.clone(),
            waiting: self.waiting.clone(),
        }
    }
}

pub struct Synchronized {
    pub reentry: usize,
    _guard: ArcReentrantMutexGuard<RawMutex, RawThreadId, ()>,
}

impl Synchronized {
    pub fn enter(&mut self) {
        self.reentry += 1;
    }

    pub fn exit(&mut self) -> bool {
        if self.reentry > 0 {
            self.reentry -= 1;
        }
        self.reentry == 0
    }

    pub fn drop_all(self) -> usize {
        drop(self._guard);
        self.reentry
    }
}
use std::sync::Arc;

use parking_lot::{RawMutex, RawThreadId};
use parking_lot::lock_api::{ArcReentrantMutexGuard, ReentrantMutex};

pub struct ObjectLock {
    mutex: Arc<ReentrantMutex<RawMutex, RawThreadId, ()>>,
}

impl ObjectLock {
    pub fn new() -> Self {
        ObjectLock {
            mutex: Arc::new(ReentrantMutex::new(())),
        }
    }

    pub fn lock(&self) -> Synchronized {
        let guard = self.mutex.lock_arc();
        Synchronized {
            reentry: 1,
            _guard: guard,
        }
    }
}

pub struct Synchronized {
    reentry: usize,
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
}
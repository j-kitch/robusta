use std::sync::{Condvar, Mutex};
use std::time::Duration;

pub struct ObjectLock {
    /// The reference to the owning object, and the number of re-enters it has performed.
    mutex: Mutex<State>,
    /// A condvar for performing waits & notifies on.
    condvar: Condvar,
    notifier: Condvar,
}

struct State {
    /// The current holder of the object lock, 0 == null, any other number is a reference to
    /// the owner.
    holder: u32,
    /// The number of re-entrant locks that the holder currently has on this lock.
    lock_count: usize,
    /// When a holder relinquishes the lock to wait, we set this field to state that the lock has
    /// been freed, but not used by another thread.
    waiting: u32,
}

impl ObjectLock {
    pub fn new() -> Self {
        ObjectLock {
            mutex: Mutex::new(State {
                holder: 0,
                lock_count: 0,
                waiting: 0,
            }),
            condvar: Condvar::new(),
            notifier: Condvar::new(),
        }
    }

    pub fn enter_monitor(&self, object: u32) {
        let mut mutex = self.mutex.lock().unwrap();

        if mutex.holder == 0 {
            // If not held, we hold.
            mutex.holder = object;
            mutex.lock_count = 1;
            mutex.waiting = 0;
        } else if mutex.holder == object {
            // If already held by us, we increment.
            mutex.lock_count += 1;
        } else {
            // We cannot hold this lock until we can.
            mutex = self.condvar.wait_while(mutex, |state| state.holder != 0).unwrap();
            mutex.holder = object;
            mutex.lock_count = 1;
            mutex.waiting = 0;
        }
    }

    pub fn exit_monitor(&self, object: u32) {
        let mut mutex = self.mutex.lock().unwrap();

        if mutex.holder != object {
            panic!("trying to exit monitor that we do not own");
        }

        if mutex.lock_count == 1 {
            // drop the entire mutex if we were the last re-entry.
            mutex.holder = 0;
            mutex.lock_count = 0;
            mutex.waiting = 0;

            // Let other threads access the lock.
            self.condvar.notify_all();
        } else {
            // drop 1 re-entry.
            mutex.lock_count -= 1;
        }
    }

    /// Completely drop our lock, notify other awaiting objects, and wait to return when we can
    /// get access again.
    pub fn wait(&self, object: u32, duration: Option<Duration>) {
        let mut mutex = self.mutex.lock().unwrap();

        if mutex.holder != object {
            panic!("trying to exit monitor that we do not own");
        }

        // Save the number of re-entrys we want to re-establish when we leave.
        let lock_count = mutex.lock_count;

        mutex.holder = 0;
        mutex.lock_count = 0;
        mutex.waiting = object;

        // Let other threads access this lock.
        self.condvar.notify_all();

        // Wait for notification on the lock, or timeout if required.
        if let Some(duration) = duration {
            mutex = self.notifier.wait_timeout(mutex, duration).unwrap().0;
        } else {
            mutex = self.notifier.wait(mutex).unwrap();
        }

        // Wait to be able to acess the mutex again.
        mutex = self.condvar.wait_while(mutex, |state| state.holder != 0).unwrap();
        mutex.holder = object;
        mutex.lock_count = lock_count;
        mutex.waiting = 0;
    }

    pub fn notify_one(&self, object: u32) {
        let mutex = self.mutex.lock().unwrap();

        if mutex.holder != object {
            panic!("trying to exit monitor that we do not own");
        }

        self.notifier.notify_one();
    }

    pub fn notify_all(&self, object: u32) {
        let mutex = self.mutex.lock().unwrap();

        if mutex.holder != object {
            panic!("trying to exit monitor that we do not own");
        }

        self.notifier.notify_all();
    }
}
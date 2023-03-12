use std::sync::{Condvar, Mutex};
use std::time::Duration;
use tracing::{debug, info};
use crate::log;

// #[derive(Clone)]
pub struct ObjectLock {
    /// The reference to the owning object, and the number of re-enters it has performed.
    mutex: Mutex<State>,
    /// A condvar for performing waits & notifies on.
    condvar: Condvar,
    notify: Mutex<()>,
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
            notify: Mutex::new(()),
            notifier: Condvar::new(),
        }
    }

    fn address(&self) -> usize {
        let pointer: *const Self = self;
        pointer as usize
    }

    pub fn enter_monitor(&self, object: u32) {
        debug!(target: log::SYNC, monitor=self.address(), "enter monitor");
        let mut mutex = self.mutex.lock().unwrap();

        if mutex.holder == 0 {
            debug!(target: log::SYNC, monitor=self.address(), "acquired monitor");
            // If not held, we hold.
            mutex.holder = object;
            mutex.lock_count = 1;
            mutex.waiting = 0;
        } else if mutex.holder == object {
            debug!(target: log::SYNC, monitor=self.address(), "incremented count");
            // If already held by us, we increment.
            mutex.lock_count += 1;
        } else {
            debug!(target: log::SYNC, monitor=self.address(), "waiting to acquire");
            // We cannot hold this lock until we can.
            mutex = self.condvar.wait_while(mutex, |state| state.holder != 0).unwrap();
            debug!(target: log::SYNC, monitor=self.address(), "acquired");
            mutex.holder = object;
            mutex.lock_count = 1;
            mutex.waiting = 0;
        }
    }

    pub fn exit_monitor(&self, object: u32) {
        debug!(target: log::SYNC, monitor=self.address(), "exiting monitor");

        let mut mutex = self.mutex.lock().unwrap();

        if mutex.holder != object {
            panic!("trying to exit monitor that we do not own");
        }

        if mutex.lock_count == 1 {
            debug!(target: log::SYNC, monitor=self.address(), "last lock, exiting fully");
            // drop the entire mutex if we were the last re-entry.
            mutex.holder = 0;
            mutex.lock_count = 0;
            mutex.waiting = 0;
            drop(mutex);

            // Let other threads access the lock.
            self.condvar.notify_all();
        } else {
            // drop 1 re-entry.
            debug!(target: log::SYNC, monitor=self.address(), "1 drop of lock count");
            mutex.lock_count -= 1;
        }
    }

    /// Completely drop our lock, notify other awaiting objects, and wait to return when we can
    /// get access again.
    pub fn wait(&self, object: u32, duration: Option<Duration>) {
        debug!(target: log::SYNC, "wait");
        let mut mutex = self.mutex.lock().unwrap();

        debug!(target: log::SYNC, "wait 2");
        if mutex.holder != object {
            panic!("trying to exit monitor that we do not own");
        }
        debug!(target: log::SYNC, "wait 3");

        // Save the number of re-entrys we want to re-establish when we leave.
        let lock_count = mutex.lock_count;

        debug!(target: log::SYNC, "wait 4");
        mutex.holder = 0;
        mutex.lock_count = 0;
        mutex.waiting = object;

        debug!(target: log::SYNC, "wait 5");
        drop(mutex);
        // Let other threads access this lock.
        self.condvar.notify_all();
        debug!(target: log::SYNC, "wait 6");

        // Wait for notification on the lock, or timeout if required.
        {
            let notify = self.notify.lock().unwrap();
        debug!(target: log::SYNC, "wait 7");
            if let Some(duration) = duration {
                let _guard = self.notifier.wait_timeout(notify, duration).unwrap().0;
            } else {
        debug!(target: log::SYNC, "wait 8");
                let _guard = self.notifier.wait(notify).expect("error occurred here!");
                info!("got here");
            }
        }

        // Wait to be able to acess the mutex again.
        let mut mutex = self.mutex.lock().unwrap();
        mutex = self.condvar.wait_while(mutex, |state| state.holder != 0).unwrap();
        mutex.holder = object;
        mutex.lock_count = lock_count;
        mutex.waiting = 0;
    }

    pub fn notify_one(&self, object: u32) {
        debug!(target: log::SYNC, "notify");
        let mutex = self.mutex.lock().unwrap();

        if mutex.holder != object {
            panic!("trying to exit monitor that we do not own");
        }
        drop(mutex);

        self.notifier.notify_one();
    }

    pub fn notify_all(&self, object: u32) {
        debug!(target: log::SYNC, "notify_all");
        let mutex = self.mutex.lock().unwrap();

        if mutex.holder != object {
            panic!("trying to exit monitor that we do not own");
        }
        drop(mutex);

        self.notifier.notify_all();
    }
}
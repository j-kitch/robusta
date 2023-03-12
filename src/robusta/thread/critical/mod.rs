// use std::sync::{Mutex, MutexGuard};
//
// pub struct CriticalLock {
//     mutex: Mutex<()>,
// }
//
// impl CriticalLock {
//     pub fn new() -> Self {
//         CriticalLock { mutex: Mutex::new(()) }
//     }
//
//     pub fn acquire<'a>(&'a self) -> impl Drop + 'a {
//         self.mutex.lock().unwrap()
//     }
// }
use crate::thread::Thread;

pub fn monitor_enter(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let lock_ref = frame.operand_stack.pop().reference();
    thread.safe.enter();
    thread.enter_monitor(lock_ref);
    thread.safe.exit();
}

pub fn monitor_exit(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let lock_ref = frame.operand_stack.pop().reference();
    thread.exit_monitor(lock_ref);
}
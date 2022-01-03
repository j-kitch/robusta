use crate::thread::Thread;

pub fn category_1(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    current.op_stack.pop_int();
}

pub fn category_2(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    current.op_stack.pop_long();
}

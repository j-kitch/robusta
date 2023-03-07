use crate::thread::Thread;

pub fn pop(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value = frame.operand_stack.pop_cat_one();

}
use crate::thread::Thread;

pub fn pop(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value = frame.operand_stack.pop();

    if value.category() > 1 {
        panic!("pop should only pop category 1")
    }
}
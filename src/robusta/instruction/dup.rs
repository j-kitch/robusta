use crate::thread::Thread;

pub fn dup(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let value = cur_frame.operand_stack.pop();

    cur_frame.operand_stack.push(value);
    cur_frame.operand_stack.push(value);
}
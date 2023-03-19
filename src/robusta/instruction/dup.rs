use crate::thread::Thread;

pub fn dup(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let value = cur_frame.operand_stack.pop();

    cur_frame.operand_stack.push(value);
    cur_frame.operand_stack.push(value);
}

pub fn dup_x1(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let value1 = frame.operand_stack.pop();
    let value2 = frame.operand_stack.pop();

    frame.operand_stack.push(value1);
    frame.operand_stack.push(value2);
    frame.operand_stack.push(value1);
}
use crate::thread::Thread;

pub fn dup(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let value = cur_frame.operand_stack.pop();

    cur_frame.operand_stack.push(value);
    cur_frame.operand_stack.push(value);
}

pub fn dup2(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let value = cur_frame.operand_stack.pop();
    if value.category() == 1 {
        let value1 = value;
        let value2 = cur_frame.operand_stack.pop();

        cur_frame.operand_stack.push(value2);
        cur_frame.operand_stack.push(value1);
        cur_frame.operand_stack.push(value2);
        cur_frame.operand_stack.push(value1);
    } else {
        cur_frame.operand_stack.push(value);
        cur_frame.operand_stack.push(value);
    }
}

pub fn dup_x1(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let value1 = frame.operand_stack.pop();
    let value2 = frame.operand_stack.pop();

    frame.operand_stack.push(value1);
    frame.operand_stack.push(value2);
    frame.operand_stack.push(value1);
}
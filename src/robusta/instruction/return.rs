use crate::thread::Thread;

pub fn a_return(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let reference = cur_frame.operand_stack.pop();

    thread.stack.pop();
    let cur_frame = thread.stack.last_mut().unwrap();

    cur_frame.operand_stack.push(reference);
}

pub fn i_return(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let int = cur_frame.operand_stack.pop();

    thread.stack.pop();
    let cur_frame = thread.stack.last_mut().unwrap();

    cur_frame.operand_stack.push(int);
}
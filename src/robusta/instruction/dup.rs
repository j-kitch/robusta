use crate::thread::Thread;

pub fn dup(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let value = cur_frame.operand_stack.pop_cat_one();

    cur_frame.operand_stack.push_cat_one(value);
    cur_frame.operand_stack.push_cat_one(value);
}
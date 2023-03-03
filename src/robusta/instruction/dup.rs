use crate::java::Value;
use crate::thread::Thread;

pub fn dup(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let value = cur_frame.operand_stack.pop();

    match &value {
        Value::Long(_) | Value::Double(_) => panic!("dup cannot be called with category type 2"),
        _ => {}
    }

    cur_frame.operand_stack.push(value);
    cur_frame.operand_stack.push(value);
}
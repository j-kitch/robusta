use crate::java::{Int, Value};
use crate::thread::Thread;

pub fn iconst_n(thread: &mut Thread, int: i32) {
    let cur_frame = thread.stack.last_mut().unwrap();

    cur_frame.operand_stack.push(Value::Int(Int(int)))
}
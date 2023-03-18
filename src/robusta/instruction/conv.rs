use crate::java::{Float, Long, Value};
use crate::thread::Thread;

pub fn int_to_float(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let int = frame.operand_stack.pop().int();
    let float = int.0 as f32;
    frame.operand_stack.push(Value::Float(Float(float)));
}

pub fn int_to_long(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let int = frame.operand_stack.pop().int();
    let long = int.0 as i64;
    frame.operand_stack.push(Value::Long(Long(long)));
}
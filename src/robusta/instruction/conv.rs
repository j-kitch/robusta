use crate::java::{Float, Value};
use crate::thread::Thread;

pub fn int_to_float(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let int = frame.operand_stack.pop().int();
    let float = int.0 as f32;
    frame.operand_stack.push(Value::Float(Float(float)));
}
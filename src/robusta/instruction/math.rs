use crate::java::{Int, Value};
use crate::thread::Thread;

pub fn i_add(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value2 = frame.operand_stack.pop().int();
    let value1 = frame.operand_stack.pop().int();

    let (result, _) = value1.0.overflowing_add(value2.0);

    frame.operand_stack.push(Value::Int(Int(result)))
}
use crate::java::{Int, Value};
use crate::thread::Thread;

pub fn pop(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    frame.operand_stack.pop();
}

pub fn sipush(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let short = frame.read_i16() as i32;
    frame.operand_stack.push(Value::Int(Int(short)));
}

pub fn bipush(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let short = frame.read_i8() as i32;
    frame.operand_stack.push(Value::Int(Int(short)));
}
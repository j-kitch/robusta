use crate::java::{Float, Int, Long, Value};
use crate::thread::Thread;

pub fn int_to_char(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let int = frame.operand_stack.pop().int();
    let bytes = int.0.to_be_bytes();
    let mut char_bytes = [0; 2];
    char_bytes[0] = bytes[2];
    char_bytes[1] = bytes[3];
    let char = u16::from_be_bytes(char_bytes);
    frame.operand_stack.push(Value::Int(Int(char as i32)));
}

pub fn int_to_float(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let int = frame.operand_stack.pop().int();
    let float = int.0 as f32;
    frame.operand_stack.push(Value::Float(Float(float)));
}

pub fn float_to_int(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let float = frame.operand_stack.pop().float();
    let int = float.0 as i32;
    frame.operand_stack.push(Value::Int(Int(int)));
}

pub fn int_to_long(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let int = frame.operand_stack.pop().int();
    let long = int.0 as i64;
    frame.operand_stack.push(Value::Long(Long(long)));
}
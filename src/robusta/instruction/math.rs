use crate::java::{ Int, Value};
use crate::thread::Thread;

pub fn i_add(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value2 = frame.operand_stack.pop().int();
    let value1 = frame.operand_stack.pop().int();

    let (result, _) = value1.0.overflowing_add(value2.0);

    frame.operand_stack.push(Value::Int(Int(result)));
}

pub fn i_inc(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u8();
    let constant = frame.read_i8();

    let value = frame.local_vars.load_cat_one(index as u16).int();
    let (value, _) = value.0.overflowing_add(constant as i32);

    frame.local_vars.store_value(index as u16, Value::Int(Int(value)))
}
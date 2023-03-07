use crate::java::{CategoryOne, Int, Value};
use crate::thread::Thread;

pub fn i_add(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value2 = frame.operand_stack.pop_cat_one().int();
    let value1 = frame.operand_stack.pop_cat_one().int();

    let (result, _) = value1.0.overflowing_add(value2.0);

    frame.operand_stack.push_cat_one(CategoryOne { int: Int(result) });
}

pub fn i_inc(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u8();
    let constant = frame.read_i8();

    let value = frame.local_vars.load_int(index as u16);
    let (value, _) = value.0.overflowing_add(constant as i32);

    frame.local_vars.store_value(index as u16, Value::Int(Int(value)))
}
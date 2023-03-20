use crate::java::{Float, Int, Long, Value};
use crate::thread::Thread;

pub fn i_neg(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value = frame.operand_stack.pop().int();

    let result = -value.0;

    frame.operand_stack.push(Value::Int(Int(result)));
}

pub fn i_add(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value2 = frame.operand_stack.pop().int();
    let value1 = frame.operand_stack.pop().int();

    let (result, _) = value1.0.overflowing_add(value2.0);

    frame.operand_stack.push(Value::Int(Int(result)));
}

pub fn l_add(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value2 = frame.operand_stack.pop().long();
    let value1 = frame.operand_stack.pop().long();

    let (result, _) = value1.0.overflowing_add(value2.0);

    frame.operand_stack.push(Value::Long(Long(result)));
}

pub fn i_sub(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value2 = frame.operand_stack.pop().int();
    let value1 = frame.operand_stack.pop().int();

    let (result, _) = value1.0.overflowing_sub(value2.0);

    frame.operand_stack.push(Value::Int(Int(result)));
}

pub fn l_sub(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value2 = frame.operand_stack.pop().long();
    let value1 = frame.operand_stack.pop().long();

    let (result, _) = value1.0.overflowing_sub(value2.0);

    frame.operand_stack.push(Value::Long(Long(result)));
}

pub fn i_mul(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value2 = frame.operand_stack.pop().int();
    let value1 = frame.operand_stack.pop().int();

    let result = value1.0.overflowing_mul(value2.0).0;

    frame.operand_stack.push(Value::Int(Int(result)));
}

pub fn f_mul(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value2 = frame.operand_stack.pop().float();
    let value1 = frame.operand_stack.pop().float();

    let result = value1.0 * value2.0;

    frame.operand_stack.push(Value::Float(Float(result)));
}

pub fn i_inc(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u8();
    let constant = frame.read_i8();

    let value = frame.local_vars.load_cat_one(index as u16).int();
    let (value, _) = value.0.overflowing_add(constant as i32);

    frame.local_vars.store_value(index as u16, Value::Int(Int(value)))
}

pub fn ixor(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value2 = frame.operand_stack.pop().int().0;
    let value1 = frame.operand_stack.pop().int().0;

    let result = value1 ^ value2;

    frame.operand_stack.push(Value::Int(Int(result)));
}

pub fn ior(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value2 = frame.operand_stack.pop().int().0;
    let value1 = frame.operand_stack.pop().int().0;

    let result = value1 | value2;

    frame.operand_stack.push(Value::Int(Int(result)));
}

pub fn iushr(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value2 = frame.operand_stack.pop().int().0;
    let value1 = frame.operand_stack.pop().int().0;

    let s = value2 & 0b11111;

    let unsigned_value_1 = u32::from_be_bytes(value1.to_be_bytes());
    let unsigned_result = unsigned_value_1 >> s;
    let result = i32::from_be_bytes(unsigned_result.to_be_bytes());

    frame.operand_stack.push(Value::Int(Int(result)));
}

pub fn ishl(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value2 = frame.operand_stack.pop().int().0;
    let value1 = frame.operand_stack.pop().int().0;

    let s = value2 & 0b11111;

    let result = value1 << s;

    frame.operand_stack.push(Value::Int(Int(result)));
}

pub fn ishr(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value2 = frame.operand_stack.pop().int().0;
    let value1 = frame.operand_stack.pop().int().0;

    let s = value2 & 0b11111;

    let result = value1 >> s;

    frame.operand_stack.push(Value::Int(Int(result)));
}

pub fn lshl(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value2 = frame.operand_stack.pop().int().0;
    let value1 = frame.operand_stack.pop().long().0;

    let s = value2 & 0b111111;

    let result = value1 << s;

    frame.operand_stack.push(Value::Long(Long(result)));
}

pub fn iand(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let value2 = frame.operand_stack.pop().int().0;
    let value1 = frame.operand_stack.pop().int().0;

    let result = value1 & value2;

    frame.operand_stack.push(Value::Int(Int(result)));
}

pub fn land(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value2 = frame.operand_stack.pop().long().0;
    let value1 = frame.operand_stack.pop().long().0;

    let result = value1 & value2;

    frame.operand_stack.push(Value::Long(Long(result)));
}

pub fn irem(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value2 = frame.operand_stack.pop().int().0;
    let value1 = frame.operand_stack.pop().int().0;

    let result = value1 % value2;

    frame.operand_stack.push(Value::Int(Int(result)));
}
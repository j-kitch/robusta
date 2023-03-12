use crate::java::Value;
use crate::thread::Thread;

pub fn istore(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u8();
    let value = frame.operand_stack.pop().int();
    frame.local_vars.store_value(index as u16, Value::Int(value));
}

pub fn astore(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u8();
    let value = frame.operand_stack.pop().reference();
    frame.local_vars.store_value(index as u16, Value::Reference(value));
}

pub fn iload(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u8();
    let value = frame.local_vars.load_cat_one(index as u16).int();
    frame.operand_stack.push_value(Value::Int(value));
}

pub fn aload(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u8();
    let value = frame.local_vars.load_cat_one(index as u16).reference();
    frame.operand_stack.push_value(Value::Reference(value));
}

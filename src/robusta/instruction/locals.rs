use crate::java::Value;
use crate::thread::Thread;

/// astore_<n>
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.astore_n).
pub fn astore_n(thread: &mut Thread, n: u16) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let value = cur_frame.operand_stack.pop();

    cur_frame.local_vars.store_value(n, Value::Reference(value.reference()));
}

/// istore_<n>
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.istore_n).
pub fn istore_n(thread: &mut Thread, n: u16) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let value = cur_frame.operand_stack.pop();

    cur_frame.local_vars.store_value(n, value);
}

pub fn lstore_n(thread: &mut Thread, n: u16) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let value = cur_frame.operand_stack.pop();

    cur_frame.local_vars.store_value(n, value);
}


/// iload_<n>
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.iload_n).
pub fn iload_n(thread: &mut Thread, n: u16) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let int = cur_frame.local_vars.load_cat_one(n).int();

    cur_frame.operand_stack.push(Value::Int(int));
}

pub fn lload_n(thread: &mut Thread, n: u16) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let long = cur_frame.local_vars.load_value(n);

    cur_frame.operand_stack.push(long);
}

/// aload_<n>
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.aload_n).
pub fn aload_n(thread: &mut Thread, n: u16) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let reference = cur_frame.local_vars.load_cat_one(n).reference();

    cur_frame.operand_stack.push(Value::Reference(reference));
}

pub fn istore(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u8();
    let value = frame.operand_stack.pop().int();
    frame.local_vars.store_value(index as u16, Value::Int(value));
}

pub fn lstore(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u8();
    let value = frame.operand_stack.pop().long();
    frame.local_vars.store_value(index as u16, Value::Long(value));
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
    frame.operand_stack.push(Value::Int(value));
}

pub fn fload_n(thread: &mut Thread, n: u16) {
    let frame = thread.stack.last_mut().unwrap();
    let float = frame.local_vars.load_value(n).float();
    frame.operand_stack.push(Value::Float(float));
}

pub fn lload(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u8();
    let value = frame.local_vars.load_value(index as u16).long();
    frame.operand_stack.push(Value::Long(value));
}

pub fn aload(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u8();
    let value = frame.local_vars.load_cat_one(index as u16).reference();
    frame.operand_stack.push(Value::Reference(value));
}

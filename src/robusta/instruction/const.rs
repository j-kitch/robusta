use crate::java::{Int, Long, Reference, Value};
use crate::thread::Thread;

/// Instruction `ldc`
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.ldc).
pub fn load_constant(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u8() as u16;

    let frame = thread.stack.last_mut().unwrap();
    let value = thread.runtime.method_area.resolve_category_one(frame.const_pool, index);

    let frame = thread.stack.last_mut().unwrap();
    frame.operand_stack.push(value);
}

pub fn load_constant_wide(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let const_idx = frame.read_u16();

    let frame = thread.stack.last_mut().unwrap();
    let const_value = thread.runtime.method_area.resolve_category_one(frame.const_pool, const_idx);

    let frame = thread.stack.last_mut().unwrap();
    frame.operand_stack.push(const_value);
}

pub fn load_constant_cat_2_wide(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let const_idx = frame.read_u16();

    let frame = thread.stack.last_mut().unwrap();
    let const_value = thread.runtime.method_area.resolve_category_two(frame.const_pool, const_idx);

    let frame = thread.stack.last_mut().unwrap();
    frame.operand_stack.push(const_value);
}

pub fn iconst_n(thread: &mut Thread, int: i32) {
    let cur_frame = thread.stack.last_mut().unwrap();

    cur_frame.operand_stack.push(Value::Int(Int(int)));
}

pub fn lconst_n(thread: &mut Thread, long: i64) {
    let cur_frame = thread.stack.last_mut().unwrap();

    cur_frame.operand_stack.push(Value::Long(Long(long)));
}

pub fn aconst_null(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    cur_frame.operand_stack.push(Value::Reference(Reference(0)));
}
use tracing::trace;
use crate::java::{ Int, Value};
use crate::log;
use crate::thread::Thread;

/// Instruction `ldc`
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.ldc).
pub fn load_constant(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u8() as u16;

    trace!(
        target: log::INSTR,
        pc=frame.pc.overflowing_sub(3).0,
        opcode="ldc",
        index
    );

    // let _guard = thread.critical_lock.acquire();
    let frame = thread.stack.last_mut().unwrap();
    let value = thread.runtime.method_area.resolve_category_one(frame.const_pool, index);

    let frame = thread.stack.last_mut().unwrap();
    frame.operand_stack.push_value(value);
}

pub fn load_constant_wide(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let const_idx = frame.read_u16();
    trace!(
        target: log::INSTR,
        pc=frame.pc-3,
        opcode="ldc_w",
        index=const_idx,
    );

    let frame = thread.stack.last_mut().unwrap();
    let const_value = thread.runtime.method_area.resolve_category_one(frame.const_pool, const_idx);

    let frame = thread.stack.last_mut().unwrap();
    frame.operand_stack.push_value(const_value);
}

pub fn load_constant_cat_2_wide(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let const_idx = frame.read_u16();
    trace!(
        target: log::INSTR,
        pc=frame.pc-3,
        opcode="ldc2_w",
        index=const_idx,
    );

    let frame = thread.stack.last_mut().unwrap();
    let const_value = thread.runtime.method_area.resolve_category_two(frame.const_pool, const_idx);

    let frame = thread.stack.last_mut().unwrap();
    frame.operand_stack.push_value(const_value);
}

pub fn iconst_n(thread: &mut Thread, int: i32) {
    let cur_frame = thread.stack.last_mut().unwrap();

    trace!(
        target: log::INSTR,
        pc=cur_frame.pc-1,
        opcode=format!("iconst_{}", int)
    );

    cur_frame.operand_stack.push_value(Value::Int(Int(int)));
}
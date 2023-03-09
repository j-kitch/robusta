use tracing::trace;
use crate::java::{CategoryOne, Int};
use crate::log;
use crate::thread::Thread;

pub fn load_constant_wide(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let const_idx = frame.read_u16();
    trace!(
        target: log::INSTR,
        pc=frame.pc-1,
        opcode="ldc_w",
        index=const_idx,
    );

    let const_value = thread.runtime.method_area.resolve_category_one(frame.const_pool, const_idx);

    frame.operand_stack.push_cat_one(const_value);
}

pub fn iconst_n(thread: &mut Thread, int: i32) {
    let cur_frame = thread.stack.last_mut().unwrap();

    trace!(
        target: log::INSTR,
        pc=cur_frame.pc-1,
        opcode=format!("iconst_{}", int)
    );

    cur_frame.operand_stack.push_cat_one(CategoryOne { int: Int(int) });
}
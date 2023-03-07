use tracing::trace;
use crate::java::{CategoryOne, Int};
use crate::log;
use crate::thread::Thread;

pub fn iconst_n(thread: &mut Thread, int: i32) {
    let cur_frame = thread.stack.last_mut().unwrap();

    trace!(
        target: log::INSTR,
        pc=cur_frame.pc-1,
        opcode=format!("iconst_{}", int)
    );

    cur_frame.operand_stack.push_cat_one(CategoryOne { int: Int(int) });
}
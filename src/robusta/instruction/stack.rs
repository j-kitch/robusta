use crate::java::{CategoryOne, Int};
use crate::thread::Thread;

pub fn pop(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    frame.operand_stack.pop_cat_one();
}

pub fn sipush(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let short = frame.read_i16() as i32;
    frame.operand_stack.push_cat_one(CategoryOne { int: Int(short) });
}
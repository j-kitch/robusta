use tracing::trace;
use crate::java::{CategoryOne, Int};
use crate::log;
use crate::thread::Thread;

pub fn array_length(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    trace!(
        target: log::INSTR,
        pc=cur_frame.pc-1,
        opcode="arraylength"
    );

    let arr_ref = unsafe { cur_frame.operand_stack.pop_cat_one().reference };

    let arr = thread.runtime.heap.get_array(arr_ref);
    let arr_length = arr.length();

    cur_frame.operand_stack.push_cat_one(CategoryOne { int: arr_length });
}

pub fn char_array_load(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.operand_stack.pop_cat_one().int();

    trace!(
        target: log::INSTR,
        pc=frame.pc-1,
        opcode="caload"
    );

    let arr_ref = frame.operand_stack.pop_cat_one().reference();
    let arr = thread.runtime.heap.get_array(arr_ref);
    let char_array = arr.as_chars_slice();

    let char = char_array[index.0 as usize];
    let char_int = Int(char as i32);

    frame.operand_stack.push_cat_one(CategoryOne { int: char_int });
}

pub fn char_array_store(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    trace!(
        target: log::INSTR,
        pc=frame.pc-1,
        opcode="castore"
    );

    let value = frame.operand_stack.pop_cat_one();
    let index = frame.operand_stack.pop_cat_one().int();
    let arr_ref = frame.operand_stack.pop_cat_one().reference();

    let arr = thread.runtime.heap.get_array(arr_ref);

    arr.set_element(index, value);
}
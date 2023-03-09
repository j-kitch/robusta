use tracing::trace;
use crate::heap::allocator::ArrayType;
use crate::java::{CategoryOne, Int};
use crate::log;
use crate::thread::Thread;

pub fn a_new_array(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let class_idx = frame.read_u16();
    let _ = thread.runtime.method_area.resolve_class(frame.const_pool, class_idx);
    let count = frame.operand_stack.pop_cat_one().int();
    let array_ref = thread.runtime.heap.new_array(ArrayType::Reference, count);
    frame.operand_stack.push_cat_one(CategoryOne { reference: array_ref });
}

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

pub fn a_array_store(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    trace!(
        target: log::INSTR,
        pc=frame.pc-1,
        opcode="aastore"
    );

    let value = frame.operand_stack.pop_cat_one();
    let index = frame.operand_stack.pop_cat_one().int();
    let arr_ref = frame.operand_stack.pop_cat_one().reference();

    let arr = thread.runtime.heap.get_array(arr_ref);

    arr.set_element(index, value);
}
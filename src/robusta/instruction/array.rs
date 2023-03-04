use crate::java::{Int, Value};
use crate::thread::Thread;

pub fn array_length(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let arr_ref = if let Value::Reference(reference) = cur_frame.operand_stack.pop() {
        reference
    } else {
        panic!("Expected reference")
    };

    let arr = thread.runtime.heap.load_array(arr_ref);
    let arr_length = arr.length();

    cur_frame.operand_stack.push(Value::Int(arr_length));
}

pub fn char_array_load(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.operand_stack.pop().int();

    let arr_ref = frame.operand_stack.pop().reference();
    let arr = thread.runtime.heap.load_array(arr_ref);
    let char_array = arr.as_chars_slice();

    let char = char_array[index.0 as usize];
    let char_int = Int(char as i32);

    frame.operand_stack.push(Value::Int(char_int));
}

pub fn char_array_store(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value = frame.operand_stack.pop();
    let index = frame.operand_stack.pop().int();
    let arr_ref = frame.operand_stack.pop().reference();

    let arr = thread.runtime.heap.load_array(arr_ref);

    arr.set_element(index, value);
}
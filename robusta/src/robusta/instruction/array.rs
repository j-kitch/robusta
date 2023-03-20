use crate::java::{ Int, Value};
use crate::thread::Thread;

pub fn a_new_array(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let const_pool = frame.const_pool;
    let class_idx = frame.read_u16();
    let class = thread.runtime.method_area.resolve_class(const_pool, class_idx);
    let frame = thread.stack.last_mut().unwrap();
    let count = frame.operand_stack.pop().int();
    let array_ref = thread.runtime.heap.new_array(class, count);
    let frame = thread.stack.last_mut().unwrap();
    frame.operand_stack.push(Value::Reference(array_ref));
}

pub fn array_length(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let arr_ref = cur_frame.operand_stack.pop().reference();

    let arr = thread.runtime.heap.get_array(arr_ref);
    let arr_length = arr.length();

    cur_frame.operand_stack.push(Value::Int(arr_length));
}

pub fn char_array_load(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.operand_stack.pop().int();
    let arr_ref = frame.operand_stack.pop().reference();
    let arr = thread.runtime.heap.get_array(arr_ref);
    let char_array = arr.as_chars_slice();

    let char = char_array[index.0 as usize];
    let char_int = Int(char as i32);

    frame.operand_stack.push(Value::Int(char_int));
}

pub fn byte_array_store(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value = frame.operand_stack.pop();
    let index = frame.operand_stack.pop().int();
    let arr_ref = frame.operand_stack.pop().reference();

    let arr = thread.runtime.heap.get_array(arr_ref);

    arr.set_element(index, value);
}

pub fn char_array_store(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value = frame.operand_stack.pop();
    let index = frame.operand_stack.pop().int();
    let arr_ref = frame.operand_stack.pop().reference();

    let arr = thread.runtime.heap.get_array(arr_ref);

    arr.set_element(index, value);
}

pub fn int_array_store(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value = frame.operand_stack.pop();
    let index = frame.operand_stack.pop().int();
    let arr_ref = frame.operand_stack.pop().reference();

    let arr = thread.runtime.heap.get_array(arr_ref);

    arr.set_element(index, value);
}

pub fn a_array_store(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let value = frame.operand_stack.pop();
    let index = frame.operand_stack.pop().int();
    let arr_ref = frame.operand_stack.pop().reference();

    let arr = thread.runtime.heap.get_array(arr_ref);

    arr.set_element(index, value);
}

pub fn a_array_load(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let index = frame.operand_stack.pop().int();
    let arr_ref = frame.operand_stack.pop().reference();

    let arr = thread.runtime.heap.get_array(arr_ref);

    let elem = arr.get_element(index);

    frame.operand_stack.push(elem);
}

pub fn int_array_load(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let index = frame.operand_stack.pop().int();
    let arr_ref = frame.operand_stack.pop().reference();

    let arr = thread.runtime.heap.get_array(arr_ref);

    let elem = arr.get_element(index);

    frame.operand_stack.push(elem);
}

pub fn byte_array_load(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let index = frame.operand_stack.pop().int();
    let arr_ref = frame.operand_stack.pop().reference();

    let arr = thread.runtime.heap.get_array(arr_ref);

    let elem = arr.get_element(index);

    frame.operand_stack.push(elem);
}
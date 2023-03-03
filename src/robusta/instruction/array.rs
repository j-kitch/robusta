use crate::java::Value;
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
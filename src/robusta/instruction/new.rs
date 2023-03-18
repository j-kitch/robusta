use crate::java::{ Value};
use crate::thread::Thread;

/// Instruction `new` creates a new object in the heap.
///
/// Opcode 0xBB
///
/// For further information, see [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.new).
pub fn new(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let class_idx = cur_frame.read_u16();

    let cur_frame = thread.stack.last_mut().unwrap();
    let class = thread.runtime.method_area.resolve_class(cur_frame.const_pool, class_idx);
    let class = unsafe { class.as_ref().unwrap() };
    let new_ref = thread.runtime.heap.new_object(class);

    let cur_frame = thread.stack.last_mut().unwrap();
    cur_frame.operand_stack.push(Value::Reference(new_ref));
}

pub fn new_array(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let array_type = cur_frame.read_u8();

    let count = cur_frame.operand_stack.pop().int();

    let arr_ref = match array_type {
        5 => thread.runtime.heap.new_array(crate::heap::allocator::ArrayType::Char, count),
        _ => panic!("newarray has not been implemented for array type {}", array_type)
    };

    let cur_frame = thread.stack.last_mut().unwrap();
    cur_frame.operand_stack.push(Value::Reference(arr_ref));
}
use tracing::trace;
use crate::heap::allocator::ArrayType;
use crate::java::{ Int, Value};
use crate::log;
use crate::thread::Thread;

pub fn a_new_array(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let const_pool = frame.const_pool;
    let class_idx = frame.read_u16();
    thread.safe.enter();
    let _ = thread.runtime.method_area.resolve_class(const_pool, class_idx);
    thread.safe.exit();
    let frame = thread.stack.last_mut().unwrap();
    let count = frame.operand_stack.pop().int();
    thread.safe.enter();
    let array_ref = thread.runtime.heap.new_array(ArrayType::Reference, count);
    thread.safe.exit();
    let frame = thread.stack.last_mut().unwrap();
    frame.operand_stack.push_value(Value::Reference(array_ref));
}

pub fn array_length(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    trace!(
        target: log::INSTR,
        pc=cur_frame.pc-1,
        opcode="arraylength"
    );

    let arr_ref = cur_frame.operand_stack.pop().reference();

    let arr = thread.runtime.heap.get_array(arr_ref);
    let arr_length = arr.length();

    cur_frame.operand_stack.push_value(Value::Int(arr_length));
}

pub fn char_array_load(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.operand_stack.pop().int();

    trace!(
        target: log::INSTR,
        pc=frame.pc-1,
        opcode="caload"
    );

    let arr_ref = frame.operand_stack.pop().reference();
    let arr = thread.runtime.heap.get_array(arr_ref);
    let char_array = arr.as_chars_slice();

    let char = char_array[index.0 as usize];
    let char_int = Int(char as i32);

    frame.operand_stack.push_value(Value::Int(char_int));
}

pub fn char_array_store(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    trace!(
        target: log::INSTR,
        pc=frame.pc-1,
        opcode="castore"
    );

    let value = frame.operand_stack.pop();
    let index = frame.operand_stack.pop().int();
    let arr_ref = frame.operand_stack.pop().reference();

    let arr = thread.runtime.heap.get_array(arr_ref);

    arr.set_element(index, value.cat_one());
}

pub fn a_array_store(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    trace!(
        target: log::INSTR,
        pc=frame.pc-1,
        opcode="aastore"
    );

    let value = frame.operand_stack.pop();
    let index = frame.operand_stack.pop().int();
    let arr_ref = frame.operand_stack.pop().reference();

    let arr = thread.runtime.heap.get_array(arr_ref);

    arr.set_element(index, value.cat_one());
}

pub fn a_array_load(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    trace!(
        target: log::INSTR,
        pc=frame.pc-1,
        opcode="aaload"
    );

    let index = frame.operand_stack.pop().int();
    let arr_ref = frame.operand_stack.pop().reference();

    let arr = thread.runtime.heap.get_array(arr_ref);

    let elem = arr.get_element(index);

    frame.operand_stack.push_value(elem);
}
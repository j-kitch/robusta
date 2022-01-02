use crate::heap::Array;
use crate::thread::op_stack::OperandStack;
use crate::thread::Thread;

pub fn reference(thread: &mut Thread) {
    array_store(thread, Array::reference_mut, OperandStack::pop_ref)
}

pub fn byte(thread: &mut Thread) {
    array_store(thread, Array::byte_mut, |stack| stack.pop_int() as i8)
}

pub fn char(thread: &mut Thread) {
    array_store(thread, Array::char_mut, |stack| stack.pop_int() as u16)
}

pub fn short(thread: &mut Thread) {
    array_store(thread, Array::short_mut, |stack| stack.pop_int() as i16)
}

pub fn int(thread: &mut Thread) {
    array_store(thread, Array::int_mut, OperandStack::pop_int)
}

pub fn long(thread: &mut Thread) {
    array_store(thread, Array::long_mut, OperandStack::pop_long)
}

pub fn float(thread: &mut Thread) {
    array_store(thread, Array::float_mut, OperandStack::pop_float)
}

pub fn double(thread: &mut Thread) {
    array_store(thread, Array::double_mut, OperandStack::pop_double)
}

fn array_store<F, G, T>(thread: &mut Thread, get_arr: F, pop_value: G)
    where T: Copy,
          F: Fn(&mut Array) -> &mut Vec<T>,
          G: Fn(&mut OperandStack) -> T
{
    let runtime = thread.rt.as_ref().borrow();
    let current = thread.frames.current_mut();

    let value = pop_value(&mut current.op_stack);
    let index = current.op_stack.pop_int() as usize;
    let array_ref = current.op_stack.pop_ref();

    let array_object = runtime.heap.get(array_ref);
    let mut array = array_object.as_ref().borrow_mut();
    let array = get_arr(array.arr_mut());

    array[index] = value;
}

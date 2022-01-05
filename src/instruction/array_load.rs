use crate::heap::Array;
use crate::thread::op_stack::OperandStack;
use crate::thread::Thread;

pub fn reference(thread: &mut Thread) {
    array_load(thread, Array::reference, OperandStack::push_ref)
}

pub fn byte(thread: &mut Thread) {
    array_load(thread, Array::byte, |stack, value| stack.push_int(value as i32))
}

pub fn char(thread: &mut Thread) {
    array_load(thread, Array::char, |stack, value| stack.push_int(value as i32))
}

pub fn short(thread: &mut Thread) {
    array_load(thread, Array::short, |stack, value| stack.push_int(value as i32))
}

pub fn int(thread: &mut Thread) {
    array_load(thread, Array::int, OperandStack::push_int)
}

pub fn long(thread: &mut Thread) {
    array_load(thread, Array::long, OperandStack::push_long)
}

pub fn float(thread: &mut Thread) {
    array_load(thread, Array::float, OperandStack::push_float)
}

pub fn double(thread: &mut Thread) {
    array_load(thread, Array::double, OperandStack::push_double)
}

fn array_load<F, G, T>(thread: &mut Thread, get_arr: F, push_value: G)
    where T: Copy,
          F: Fn(&Array) -> &Vec<T>,
          G: Fn(&mut OperandStack, T) {
    let runtime = thread.rt.as_ref().borrow();
    let current = thread.frames.current_mut();
    let index = current.op_stack.pop_int() as usize;
    let array_ref = current.op_stack.pop_ref();

    let array_object = runtime.heap.get(array_ref);
    let array = array_object.as_ref().borrow();
    let array = get_arr(array.arr());
    let value = array[index];

    push_value(&mut current.op_stack, value);
}

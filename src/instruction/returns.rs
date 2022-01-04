use crate::thread::op_stack::OperandStack;
use crate::thread::Thread;

pub fn int(thread: &mut Thread) {
    return_value(thread, OperandStack::pop_int, OperandStack::push_int)
}

pub fn long(thread: &mut Thread) {
    return_value(thread, OperandStack::pop_long, OperandStack::push_long)
}

pub fn float(thread: &mut Thread) {
    return_value(thread, OperandStack::pop_float, OperandStack::push_float)
}

pub fn double(thread: &mut Thread) {
    return_value(thread, OperandStack::pop_double, OperandStack::push_double)
}

pub fn reference(thread: &mut Thread) {
    return_value(thread, OperandStack::pop_ref, OperandStack::push_ref)
}

pub fn none(thread: &mut Thread) {
    thread.frames.pop();
}

fn return_value<T, F, G>(thread: &mut Thread, pop: F, push: G)
    where T: Copy,
          F: Fn(&mut OperandStack) -> T,
          G: Fn(&mut OperandStack, T) {
    let current = thread.frames.current_mut();
    let result = pop(&mut current.op_stack);
    thread.frames.pop();
    let current = thread.frames.current_mut();
    push(&mut current.op_stack, result)
}

use crate::thread::op_stack::OperandStack;
use crate::thread::Thread;

pub fn int_m1(thread: &mut Thread) {
    push_const(thread, -1, OperandStack::push_int)
}

pub fn int_0(thread: &mut Thread) {
    push_const(thread, 0, OperandStack::push_int)
}

pub fn int_1(thread: &mut Thread) {
    push_const(thread, 1, OperandStack::push_int)
}

pub fn int_2(thread: &mut Thread) {
    push_const(thread, 2, OperandStack::push_int)
}

pub fn int_3(thread: &mut Thread) {
    push_const(thread, 3, OperandStack::push_int)
}

pub fn int_4(thread: &mut Thread) {
    push_const(thread, 4, OperandStack::push_int)
}

pub fn int_5(thread: &mut Thread) {
    push_const(thread, 5, OperandStack::push_int)
}

pub fn long_0(thread: &mut Thread) {
    push_const(thread, 0, OperandStack::push_long)
}

pub fn long_1(thread: &mut Thread) {
    push_const(thread, 1, OperandStack::push_long)
}

pub fn float_0(thread: &mut Thread) {
    push_const(thread, 0.0, OperandStack::push_float)
}

pub fn float_1(thread: &mut Thread) {
    push_const(thread, 1.0, OperandStack::push_float)
}

pub fn float_2(thread: &mut Thread) {
    push_const(thread, 2.0, OperandStack::push_float)
}

pub fn double_0(thread: &mut Thread) {
    push_const(thread, 0.0, OperandStack::push_double)
}

pub fn double_1(thread: &mut Thread) {
    push_const(thread, 1.0, OperandStack::push_double)
}

fn push_const<T, F>(thread: &mut Thread, value: T, push_value: F)
    where T: Copy, F: Fn(&mut OperandStack, T) {
    let current = thread.frames.current_mut();
    push_value(&mut current.op_stack, value)
}

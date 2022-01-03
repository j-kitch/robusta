use crate::thread::op_stack::OperandStack;
use crate::thread::Thread;

pub fn int_add(thread: &mut Thread) {
    binary_op(thread, |i1, i2| i1.overflowing_add(i2).0, OperandStack::pop_int, OperandStack::push_int)
}

pub fn long_add(thread: &mut Thread) {
    binary_op(thread, |i1, i2| i1.overflowing_add(i2).0, OperandStack::pop_long, OperandStack::push_long)
}

pub fn float_add(thread: &mut Thread) {
    binary_op(thread, |f1, f2| f1 + f2, OperandStack::pop_float, OperandStack::push_float)
}

pub fn double_add(thread: &mut Thread) {
    binary_op(thread, |d1, d2| d1 + d2, OperandStack::pop_double, OperandStack::push_double)
}

pub fn int_sub(thread: &mut Thread) {
    binary_op(thread, |i1, i2| i1.overflowing_sub(i2).0, OperandStack::pop_int, OperandStack::push_int)
}

pub fn long_sub(thread: &mut Thread) {
    binary_op(thread, |i1, i2| i1.overflowing_sub(i2).0, OperandStack::pop_long, OperandStack::push_long)
}

pub fn float_sub(thread: &mut Thread) {
    binary_op(thread, |f1, f2| f1 - f2, OperandStack::pop_float, OperandStack::push_float)
}

pub fn double_sub(thread: &mut Thread) {
    binary_op(thread, |d1, d2| d1 - d2, OperandStack::pop_double, OperandStack::push_double)
}

pub fn int_mul(thread: &mut Thread) {
    binary_op(thread, |i1, i2| i1.overflowing_mul(i2).0, OperandStack::pop_int, OperandStack::push_int)
}

pub fn long_mul(thread: &mut Thread) {
    binary_op(thread, |i1, i2| i1.overflowing_mul(i2).0, OperandStack::pop_long, OperandStack::push_long)
}

pub fn float_mul(thread: &mut Thread) {
    binary_op(thread, |f1, f2| f1 * f2, OperandStack::pop_float, OperandStack::push_float)
}

pub fn double_mul(thread: &mut Thread) {
    binary_op(thread, |d1, d2| d1 * d2, OperandStack::pop_double, OperandStack::push_double)
}

pub fn int_div(thread: &mut Thread) {
    binary_op(thread, |i1, i2| i1.overflowing_div(i2).0, OperandStack::pop_int, OperandStack::push_int)
}

pub fn long_div(thread: &mut Thread) {
    binary_op(thread, |i1, i2| i1.overflowing_div(i2).0, OperandStack::pop_long, OperandStack::push_long)
}

pub fn float_div(thread: &mut Thread) {
    binary_op(thread, |f1, f2| f1 / f2, OperandStack::pop_float, OperandStack::push_float)
}

pub fn double_div(thread: &mut Thread) {
    binary_op(thread, |d1, d2| d1 / d2, OperandStack::pop_double, OperandStack::push_double)
}

pub fn int_rem(thread: &mut Thread) {
    binary_op(thread, |i1, i2| i1.overflowing_rem(i2).0, OperandStack::pop_int, OperandStack::push_int)
}

pub fn long_rem(thread: &mut Thread) {
    binary_op(thread, |i1, i2| i1.overflowing_rem(i2).0, OperandStack::pop_long, OperandStack::push_long)
}

pub fn float_rem(thread: &mut Thread) {
    binary_op(thread, |f1, f2| f1 % f2, OperandStack::pop_float, OperandStack::push_float)
}

pub fn double_rem(thread: &mut Thread) {
    binary_op(thread, |d1, d2| d1 % d2, OperandStack::pop_double, OperandStack::push_double)
}

fn binary_op<T, F, G, H>(thread: &mut Thread, op: F, pop: G, push: H)
    where T: Copy,
          F: Fn(T, T) -> T,
          G: Fn(&mut OperandStack) -> T,
          H: Fn(&mut OperandStack, T) {
    let current = thread.frames.current_mut();
    let value2 = pop(&mut current.op_stack);
    let value1 = pop(&mut current.op_stack);

    let result = op(value1, value2);

    push(&mut current.op_stack, result)
}

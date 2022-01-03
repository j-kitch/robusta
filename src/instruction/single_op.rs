use std::ops::Neg;

use crate::thread::op_stack::OperandStack;
use crate::thread::Thread;

pub fn int_inc(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let idx = current.read_u8();
    let inc = current.read_i8() as i32;
    let int = current.local_vars.load_int(idx as u16);
    let (result, _) = int.overflowing_add(inc);
    current.local_vars.store_int(idx as u16, result)
}

pub fn int_neg(thread: &mut Thread) {
    neg(thread, OperandStack::pop_int, OperandStack::push_int)
}

pub fn long_neg(thread: &mut Thread) {
    neg(thread, OperandStack::pop_long, OperandStack::push_long)
}

pub fn float_neg(thread: &mut Thread) {
    neg(thread, OperandStack::pop_float, OperandStack::push_float)
}

pub fn double_neg(thread: &mut Thread) {
    neg(thread, OperandStack::pop_double, OperandStack::push_double)
}

fn neg<T, F, G>(thread: &mut Thread, pop_value: F, push_value: G)
    where T: Neg + Copy + Neg<Output = T>,
          F: Fn(&mut OperandStack) -> T,
          G: Fn(&mut OperandStack, T) {
    let current = thread.frames.current_mut();
    let value = pop_value(&mut current.op_stack);

    let result = -value;

    push_value(&mut current.op_stack, result)
}

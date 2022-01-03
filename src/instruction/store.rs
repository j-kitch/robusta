use crate::thread::local_vars::LocalVars;
use crate::thread::op_stack::OperandStack;
use crate::thread::Thread;

pub fn int(thread: &mut Thread) {
    store(thread, OperandStack::pop_int, LocalVars::store_int)
}

pub fn int_0(thread: &mut Thread) {
    store_index(thread, 0, OperandStack::pop_int, LocalVars::store_int)
}

pub fn int_1(thread: &mut Thread) {
    store_index(thread, 1, OperandStack::pop_int, LocalVars::store_int)
}

pub fn int_2(thread: &mut Thread) {
    store_index(thread, 2, OperandStack::pop_int, LocalVars::store_int)
}

pub fn int_3(thread: &mut Thread) {
    store_index(thread, 3, OperandStack::pop_int, LocalVars::store_int)
}

pub fn long(thread: &mut Thread) {
    store(thread, OperandStack::pop_long, LocalVars::store_long)
}

pub fn long_0(thread: &mut Thread) {
    store_index(thread, 0, OperandStack::pop_long, LocalVars::store_long)
}

pub fn long_1(thread: &mut Thread) {
    store_index(thread, 0, OperandStack::pop_long, LocalVars::store_long)
}

pub fn long_2(thread: &mut Thread) {
    store_index(thread, 0, OperandStack::pop_long, LocalVars::store_long)
}

pub fn long_3(thread: &mut Thread) {
    store_index(thread, 0, OperandStack::pop_long, LocalVars::store_long)
}

pub fn float(thread: &mut Thread) {
    store(thread, OperandStack::pop_float, LocalVars::store_float)
}

pub fn float_0(thread: &mut Thread) {
    store_index(thread, 0, OperandStack::pop_float, LocalVars::store_float)
}

pub fn float_1(thread: &mut Thread) {
    store_index(thread, 1, OperandStack::pop_float, LocalVars::store_float)
}

pub fn float_2(thread: &mut Thread) {
    store_index(thread, 2, OperandStack::pop_float, LocalVars::store_float)
}

pub fn float_3(thread: &mut Thread) {
    store_index(thread, 3, OperandStack::pop_float, LocalVars::store_float)
}

pub fn double(thread: &mut Thread) {
    store(thread, OperandStack::pop_double, LocalVars::store_double)
}

pub fn double_0(thread: &mut Thread) {
    store_index(thread, 0, OperandStack::pop_double, LocalVars::store_double)
}

pub fn double_1(thread: &mut Thread) {
    store_index(thread, 1, OperandStack::pop_double, LocalVars::store_double)
}

pub fn double_2(thread: &mut Thread) {
    store_index(thread, 2, OperandStack::pop_double, LocalVars::store_double)
}

pub fn double_3(thread: &mut Thread) {
    store_index(thread, 3, OperandStack::pop_double, LocalVars::store_double)
}

pub fn reference(thread: &mut Thread) {
    store(thread, OperandStack::pop_ref, LocalVars::store_ref)
}

pub fn reference_0(thread: &mut Thread) {
    store_index(thread, 0, OperandStack::pop_ref, LocalVars::store_ref)
}

pub fn reference_1(thread: &mut Thread) {
    store_index(thread, 1, OperandStack::pop_ref, LocalVars::store_ref)
}

pub fn reference_2(thread: &mut Thread) {
    store_index(thread, 2, OperandStack::pop_ref, LocalVars::store_ref)
}

pub fn reference_3(thread: &mut Thread) {
    store_index(thread, 3, OperandStack::pop_ref, LocalVars::store_ref)
}

fn store<T, F, G>(thread: &mut Thread, pop_value: F, store_var: G)
    where T: Copy,
          F: Fn(&mut OperandStack) -> T,
          G: Fn(&mut LocalVars, u16, T) {
    let current = thread.frames.current_mut();
    let index = current.read_u8() as u16;
    store_index(thread, index, pop_value, store_var)
}

fn store_index<T, F, G>(thread: &mut Thread, index: u16, pop_value: F, store_var: G)
    where T: Copy,
          F: Fn(&mut OperandStack) -> T,
          G: Fn(&mut LocalVars, u16, T) {
    let current = thread.frames.current_mut();
    let value = pop_value(&mut current.op_stack);
    store_var(&mut current.local_vars, index, value)
}

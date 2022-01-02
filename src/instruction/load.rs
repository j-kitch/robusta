use crate::thread::local_vars::LocalVars;
use crate::thread::op_stack::OperandStack;
use crate::thread::Thread;

pub fn int(thread: &mut Thread) {
    load(thread, LocalVars::load_int, OperandStack::push_int)
}

pub fn int_0(thread: &mut Thread) {
    load_index(thread, 0, LocalVars::load_int, OperandStack::push_int)
}

pub fn int_1(thread: &mut Thread) {
    load_index(thread, 1, LocalVars::load_int, OperandStack::push_int)
}

pub fn int_2(thread: &mut Thread) {
    load_index(thread, 2, LocalVars::load_int, OperandStack::push_int)
}

pub fn int_3(thread: &mut Thread) {
    load_index(thread, 3, LocalVars::load_int, OperandStack::push_int)
}

pub fn long(thread: &mut Thread) {
    load(thread, LocalVars::load_long, OperandStack::push_long)
}

pub fn long_0(thread: &mut Thread) {
    load_index(thread, 0, LocalVars::load_long, OperandStack::push_long)
}

pub fn long_1(thread: &mut Thread) {
    load_index(thread, 1, LocalVars::load_long, OperandStack::push_long)
}

pub fn long_2(thread: &mut Thread) {
    load_index(thread, 2, LocalVars::load_long, OperandStack::push_long)
}

pub fn long_3(thread: &mut Thread) {
    load_index(thread, 3, LocalVars::load_long, OperandStack::push_long)
}

pub fn float(thread: &mut Thread) {
    load(thread, LocalVars::load_float, OperandStack::push_float)
}

pub fn float_0(thread: &mut Thread) {
    load_index(thread, 0, LocalVars::load_float, OperandStack::push_float)
}

pub fn float_1(thread: &mut Thread) {
    load_index(thread, 1, LocalVars::load_float, OperandStack::push_float)
}

pub fn float_2(thread: &mut Thread) {
    load_index(thread, 2, LocalVars::load_float, OperandStack::push_float)
}

pub fn float_3(thread: &mut Thread) {
    load_index(thread, 3, LocalVars::load_float, OperandStack::push_float)
}

fn load<T, F, G>(thread: &mut Thread, load_var: F, push_var: G)
    where T: Copy,
          F: Fn(&LocalVars, u16) -> T,
          G: Fn(&mut OperandStack, T) {
    let current = thread.frames.current_mut();
    let index = current.read_u8() as u16;
    load_index(thread, index, load_var, push_var)
}

fn load_index<T, F, G>(thread: &mut Thread, index: u16, load_var: F, push_var: G)
    where T: Copy,
          F: Fn(&LocalVars, u16) -> T,
          G: Fn(&mut OperandStack, T) {
    let current = thread.frames.current_mut();
    let value = load_var(&current.local_vars, index);
    push_var(&mut current.op_stack, value)
}

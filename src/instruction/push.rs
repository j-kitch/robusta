use crate::thread::{Frame, Thread};
use crate::thread::op_stack::OperandStack;

pub fn byte(thread: &mut Thread) {
    push(thread, |frame| frame.read_i8() as i32, OperandStack::push_int)
}

pub fn short(thread: &mut Thread) {
    push(thread, |frame| frame.read_i16() as i32, OperandStack::push_int)
}

fn push<T, F, G>(thread: &mut Thread, read_value: F, push_value: G)
    where T: Copy,
          F: Fn(&mut Frame) -> T,
          G: Fn(&mut OperandStack, T) {
    let current = thread.frames.current_mut();
    let value = read_value(current);
    push_value(&mut current.op_stack, value);
}

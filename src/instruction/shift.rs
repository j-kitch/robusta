use crate::thread::op_stack::OperandStack;
use crate::thread::Thread;

pub fn int_left(thread: &mut Thread) {
    shift(thread, OperandStack::pop_int, |i, u| i.overflowing_shl(u).0, OperandStack::push_int)
}

pub fn long_left(thread: &mut Thread) {
    shift(thread, OperandStack::pop_long, |l, u| l.overflowing_shl(u).0, OperandStack::push_long)
}

pub fn int_right(thread: &mut Thread) {
    shift(thread, OperandStack::pop_int, |i, u| i.overflowing_shr(u).0, OperandStack::push_int)
}

pub fn long_right(thread: &mut Thread) {
    shift(thread, OperandStack::pop_long, |l, u| l.overflowing_shr(u).0, OperandStack::push_long)
}

pub fn int_right_unsigned(thread: &mut Thread) {
    shift(thread, OperandStack::pop_int, |i, u| to_i32(to_u32(i).overflowing_shr(u).0), OperandStack::push_int)
}

pub fn long_right_unsigned(thread: &mut Thread) {
    shift(thread, OperandStack::pop_long, |l, u| to_i64(to_u64(l).overflowing_shr(u).0), OperandStack::push_long)
}

fn to_u32(i: i32) -> u32 {
    u32::from_be_bytes(i.to_be_bytes())
}

fn to_i32(u: u32) -> i32 {
    i32::from_be_bytes(u.to_be_bytes())
}

fn to_u64(i: i64) -> u64 {
    u64::from_be_bytes(i.to_be_bytes())
}

fn to_i64(u: u64) -> i64 {
    i64::from_be_bytes(u.to_be_bytes())
}

fn shift<T, F, G, H>(thread: &mut Thread, pop: F, shift: G, push: H)
    where T: Copy,
          F: Fn(&mut OperandStack) -> T,
          G: Fn(T, u32) -> T,
          H: Fn(&mut OperandStack, T)
{
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_int();
    let value1 = pop(&mut current.op_stack);

    let s = (0x1F & value2) as u32;

    let result = shift(value1, s);

    push(&mut current.op_stack, result);
}

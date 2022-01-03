use crate::thread::op_stack::OperandStack;
use crate::thread::Thread;

pub fn int_to_byte(thread: &mut Thread) {
    convert(thread, OperandStack::pop_int, OperandStack::push_int, |i| i & 0xFF)
}

pub fn int_to_short(thread: &mut Thread) {
    convert(thread, OperandStack::pop_int, OperandStack::push_int, |i| i & 0xFF_FF)
}

pub fn int_to_char(thread: &mut Thread) {
    convert(thread, OperandStack::pop_int, OperandStack::push_int, |i| i & 0xFF_FF)
}

pub fn int_to_long(thread: &mut Thread) {
    convert(thread, OperandStack::pop_int, OperandStack::push_long, |i| i as i64)
}

pub fn int_to_float(thread: &mut Thread) {
    convert(thread, OperandStack::pop_int, OperandStack::push_float, |i| i as f32)
}

pub fn int_to_double(thread: &mut Thread) {
    convert(thread, OperandStack::pop_int, OperandStack::push_double, |i| i as f64)
}

pub fn long_to_int(thread: &mut Thread) {
    convert(thread, OperandStack::pop_long, OperandStack::push_int, low_order_int)
}

pub fn long_to_float(thread: &mut Thread) {
    convert(thread, OperandStack::pop_long, OperandStack::push_float, |l| l as f32)
}

pub fn long_to_double(thread: &mut Thread) {
    convert(thread, OperandStack::pop_long, OperandStack::push_double, |l| l as f64)
}

pub fn float_to_int(thread: &mut Thread) {
    convert(thread, OperandStack::pop_float, OperandStack::push_int, |f| f as i32)
}

pub fn float_to_long(thread: &mut Thread) {
    convert(thread, OperandStack::pop_float, OperandStack::push_long, |f| f as i64)
}

pub fn float_to_double(thread: &mut Thread) {
    convert(thread, OperandStack::pop_float, OperandStack::push_double, |f| f as f64)
}

pub fn double_to_int(thread: &mut Thread) {
    convert(thread, OperandStack::pop_double, OperandStack::push_int, |d| d as i32)
}

pub fn double_to_long(thread: &mut Thread) {
    convert(thread, OperandStack::pop_double, OperandStack::push_long, |d| d as i64)
}

pub fn double_to_float(thread: &mut Thread) {
    convert(thread, OperandStack::pop_double, OperandStack::push_float, |d| d as f32)
}

fn low_order_int(i: i64) -> i32 {
    let bytes = i.to_be_bytes();
    i32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]])
}

fn convert<T1, T2, F, G, H>(thread: &mut Thread, pop_t1: F, push_t2: G, convert: H)
    where T1: Copy,
          T2: Copy,
          F: Fn(&mut OperandStack) -> T1,
          G: Fn(&mut OperandStack, T2),
          H: Fn(T1) -> T2 {
    let current = thread.frames.current_mut();
    let t1 = pop_t1(&mut current.op_stack);
    let t2 = convert(t1);
    push_t2(&mut current.op_stack, t2);
}

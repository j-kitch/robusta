use crate::thread::Thread;

pub fn long(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_long();
    let value1 = current.op_stack.pop_long();

    let result: i32 = if value1 < value2 {
        -1
    } else if value1 == value2 {
        0
    } else {
        1
    };

    current.op_stack.push_int(result)
}

pub fn float_g(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_float();
    let value1 = current.op_stack.pop_float();

    let result: i32 = if value1.is_nan() || value2.is_nan() {
        1
    } else if value1 < value2 {
        -1
    } else if value1 == value2 {
        0
    } else {
        1
    };

    current.op_stack.push_int(result)
}

pub fn float_l(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_float();
    let value1 = current.op_stack.pop_float();

    let result: i32 = if value1.is_nan() || value2.is_nan() {
        1
    } else if value1 < value2 {
        -1
    } else if value1 == value2 {
        0
    } else {
        -1
    };

    current.op_stack.push_int(result)
}


pub fn double_g(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_double();
    let value1 = current.op_stack.pop_double();

    let result: i32 = if value1.is_nan() || value2.is_nan() {
        1
    } else if value1 < value2 {
        -1
    } else if value1 == value2 {
        0
    } else {
        1
    };

    current.op_stack.push_int(result)
}

pub fn double_l(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_double();
    let value1 = current.op_stack.pop_double();

    let result: i32 = if value1.is_nan() || value2.is_nan() {
        1
    } else if value1 < value2 {
        -1
    } else if value1 == value2 {
        0
    } else {
        -1
    };

    current.op_stack.push_int(result)
}

pub fn if_eq(thread: &mut Thread) {
    if_cond(thread, |i| i == 0)
}

pub fn if_ne(thread: &mut Thread) {
    if_cond(thread, |i| i != 0)
}

pub fn if_lt(thread: &mut Thread) {
    if_cond(thread, |i| i < 0)
}

pub fn if_le(thread: &mut Thread) {
    if_cond(thread, |i| i <= 0)
}

pub fn if_gt(thread: &mut Thread) {
    if_cond(thread, |i| i > 0)
}

pub fn if_ge(thread: &mut Thread) {
    if_cond(thread, |i| i >= 0)
}

pub fn if_int_eq(thread: &mut Thread) {
    if_int_cond(thread, |i1, i2| i1 == i2)
}

pub fn if_int_ne(thread: &mut Thread) {
    if_int_cond(thread, |i1, i2| i1 != i2)
}

pub fn if_int_lt(thread: &mut Thread) {
    if_int_cond(thread, |i1, i2| i1 < i2)
}

pub fn if_int_le(thread: &mut Thread) {
    if_int_cond(thread, |i1, i2| i1 <= i2)
}

pub fn if_int_gt(thread: &mut Thread) {
    if_int_cond(thread, |i1, i2| i1 > i2)
}

pub fn if_int_ge(thread: &mut Thread) {
    if_int_cond(thread, |i1, i2| i1 >= i2)
}

pub fn if_ref_eq(thread: &mut Thread) {
    if_reference_cond(thread, |r1, r2| r1 == r2)
}

pub fn if_ref_ne(thread: &mut Thread) {
    if_reference_cond(thread, |r1, r2| r1 != r2)
}

pub fn if_non_null(thread: &mut Thread) { if_reference_single(thread, |r| r != 0) }

pub fn if_null(thread: &mut Thread) { if_reference_single(thread, |r| r == 0) }

fn if_cond<F>(thread: &mut Thread, comp: F) where F: Fn(i32) -> bool {
    let current = thread.frames.current_mut();
    let offset = current.read_i16();
    let value = current.op_stack.pop_int();

    if comp(value) {
        let mut pc = current.pc as i64 - 3;
        pc += offset as i64;
        current.pc = pc as u32;
    }
}

fn if_int_cond<F>(thread: &mut Thread, comp: F) where F: Fn(i32, i32) -> bool {
    let current = thread.frames.current_mut();
    let offset = current.read_i16();
    let value2 = current.op_stack.pop_int();
    let value1 = current.op_stack.pop_int();

    if comp(value1, value2) {
        let mut pc = current.pc as i64 - 3;
        pc += offset as i64;
        current.pc = pc as u32;
    }
}

fn if_reference_cond<F>(thread: &mut Thread, comp: F) where F: Fn(u32, u32) -> bool {
    let current = thread.frames.current_mut();
    let offset = current.read_i16();
    let value2 = current.op_stack.pop_ref();
    let value1 = current.op_stack.pop_ref();

    if comp(value1, value2) {
        let mut pc = current.pc as i64 - 3;
        pc += offset as i64;
        current.pc = pc as u32;
    }
}

fn if_reference_single<F>(thread: &mut Thread, comp: F) where F: Fn(u32) -> bool {
    let current = thread.frames.current_mut();
    let offset = current.read_i16();
    let value = current.op_stack.pop_ref();

    if comp(value) {
        let mut pc = current.pc as i64 - 3;
        pc += offset as i64;
        current.pc = pc as u32;
    }
}

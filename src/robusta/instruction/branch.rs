use crate::thread::Thread;

pub fn if_int_cmp_ge(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let offset = frame.read_i16();

    let value2 = frame.operand_stack.pop().int();
    let value1 = frame.operand_stack.pop().int();

    if value1.0 >= value2.0 {
        let mut pc = frame.pc as i64;
        pc -= 3;
        pc += offset as i64;
        frame.pc = pc as usize;
    }
}

pub fn if_int_cmp_le(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let offset = frame.read_i16();

    let value2 = frame.operand_stack.pop().int();
    let value1 = frame.operand_stack.pop().int();

    if value1.0 <= value2.0 {
        let mut pc = frame.pc as i64;
        pc -= 3;
        pc += offset as i64;
        frame.pc = pc as usize;
    }
}

pub fn if_int_cmp_lt(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let offset = frame.read_i16();

    let value2 = frame.operand_stack.pop().int();
    let value1 = frame.operand_stack.pop().int();

    if value1.0 < value2.0 {
        let mut pc = frame.pc as i64;
        pc -= 3;
        pc += offset as i64;
        frame.pc = pc as usize;
    }
}

pub fn if_int_cmp_ne(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let offset = frame.read_i16();

    let value2 = frame.operand_stack.pop().int();
    let value1 = frame.operand_stack.pop().int();

    if value1.0 != value2.0 {
        let mut pc = frame.pc as i64;
        pc -= 3;
        pc += offset as i64;
        frame.pc = pc as usize;
    }
}

pub fn if_ref_cmp_ne(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let offset = frame.read_i16();

    let value2 = frame.operand_stack.pop().reference();
    let value1 = frame.operand_stack.pop().reference();

    if value1.0 != value2.0 {
        let mut pc = frame.pc as i64;
        pc -= 3;
        pc += offset as i64;
        frame.pc = pc as usize;
    }
}

pub fn goto(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let offset = frame.read_i16();

    let mut pc = frame.pc as i64;
    pc -= 3;
    pc += offset as i64;
    frame.pc = pc as usize;
}

pub fn if_eq(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let offset = frame.read_i16();

    let value = frame.operand_stack.pop().int();

    if value.0 == 0 {
        let mut pc = frame.pc as i64;
        pc -= 3;
        pc += offset as i64;
        frame.pc = pc as usize;
    }
}

pub fn if_lt(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let offset = frame.read_i16();
    let value = frame.operand_stack.pop().int();

    if value.0 < 0 {
        let mut pc = frame.pc as i64;
        pc -= 3;
        pc += offset as i64;
        frame.pc = pc as usize;
    }
}

pub fn if_ne(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let offset = frame.read_i16();

    let value = frame.operand_stack.pop().int();

    if value.0 != 0 {
        let mut pc = frame.pc as i64;
        pc -= 3;
        pc += offset as i64;
        frame.pc = pc as usize;
    }
}

pub fn if_null(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let offset = frame.read_i16();

    let value = frame.operand_stack.pop().reference();

    if value.0 == 0 {
        let mut pc = frame.pc as i64;
        pc -= 3;
        pc += offset as i64;
        frame.pc = pc as usize;
    }
}

pub fn if_non_null(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let offset = frame.read_i16();

    let value = frame.operand_stack.pop().reference();

    if value.0 != 0 {
        let mut pc = frame.pc as i64;
        pc -= 3;
        pc += offset as i64;
        frame.pc = pc as usize;
    }
}
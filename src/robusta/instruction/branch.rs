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
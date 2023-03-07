use tracing::trace;
use crate::log;
use crate::thread::Thread;

pub fn if_int_cmp_ge(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let offset = frame.read_i16();

    trace!(
        target: log::INSTR,
        pc=frame.pc-3,
        opcode="if_icmpge",
        offset
    );

    let value2 = frame.operand_stack.pop_cat_one().int();
    let value1 = frame.operand_stack.pop_cat_one().int();

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

    trace!(
        target: log::INSTR,
        pc=frame.pc-3,
        opcode="if_icmple",
        offset
    );

    let value2 = frame.operand_stack.pop_cat_one().int();
    let value1 = frame.operand_stack.pop_cat_one().int();

    if value1.0 <= value2.0 {
        let mut pc = frame.pc as i64;
        pc -= 3;
        pc += offset as i64;
        frame.pc = pc as usize;
    }
}

pub fn goto(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let offset = frame.read_i16();

    trace!(
        target: log::INSTR,
        pc=frame.pc-3,
        opcode="goto",
        offset
    );

    let mut pc = frame.pc as i64;
    pc -= 3;
    pc += offset as i64;
    frame.pc = pc as usize;
}
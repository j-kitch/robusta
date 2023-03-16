use tracing::trace;
use crate::log;
use crate::thread::Thread;

pub fn dup(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    trace!(
        target: log::INSTR,
        pc=cur_frame.pc-1,
        opcode="dup"
    );

    let value = cur_frame.operand_stack.pop();

    cur_frame.operand_stack.push(value);
    cur_frame.operand_stack.push(value);
}
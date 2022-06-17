use crate::thread::Thread;

pub fn goto_wide(thread: &mut Thread) {
    let mut current = thread.frames.current_mut();
    let offset = current.read_i32();

    let mut pc = current.pc as i64;
    pc += offset as i64;
    current.pc = pc as u32;
}

pub fn jump_subroutine_wide(thread: &mut Thread) {
    let mut current = thread.frames.current_mut();
    let offset = current.read_i32();
    current.op_stack.push_return_address(current.pc);

    let mut pc = current.pc as i64 - 5;
    pc += offset as i64;
    current.pc = pc as u32;
}

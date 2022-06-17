use std::collections::HashMap;
use crate::thread::Thread;

pub fn subroutine(thread: &mut Thread) {
    let mut current = thread.frames.current_mut();
    let offset = current.read_i16();
    let next_instr_offset = current.pc;
    let this_instr_offset = next_instr_offset - 3;

    current.op_stack.push_return_address(next_instr_offset);

    let mut pc = this_instr_offset as i64;
    pc += offset as i64;
    current.pc = pc as u32;
}

pub fn ret(thread: &mut Thread) {
    let mut current = thread.frames.current_mut();
    let idx = current.read_u8();

    let new_pc = current.local_vars.load_return_address(idx as u16);

    current.pc = new_pc;
}

pub fn table_switch(thread: &mut Thread) {
    let mut current = thread.frames.current_mut();
    let opcode = current.pc - 1;

    let alignment = current.pc % 4;
    let padding = 4 - alignment;
    current.pc += padding;

    let default = current.read_i32();
    let low = current.read_i32();
    let high = current.read_i32();

    let num_offsets = (high - low + 1) as usize;
    let offsets: Vec<i32> = (0..num_offsets)
        .map(|_| current.read_i32())
        .collect();

    let index = current.op_stack.pop_int();

    let pc = if index < low || index > high {
        opcode as i64 + default as i64
    } else {
        let offset = (index - low) as usize;
        opcode as i64 + offsets[offset] as i64
    };

    current.pc = opcode as u32;
}

pub fn lookup_switch(thread: &mut Thread) {
    let mut current = thread.frames.current_mut();
    let opcode = current.pc - 1;

    let alignment = current.pc % 4;
    let padding = 4 - alignment;
    current.pc += padding;

    let default = current.read_i32();
    let n_pairs = current.read_i32();
    let pairs: HashMap<i32, i32> = (0..n_pairs)
        .map(|_| (current.read_i32(), current.read_i32()))
        .collect();

    let key = current.op_stack.pop_int();

    let offset = pairs.get(&key).map_or(default, Clone::clone);

    let mut pc = opcode as i64;
    pc += offset as i64;
    current.pc = pc as u32;
}

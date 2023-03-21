use crate::java::{Int, Value};
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

pub fn if_int_cmp_eq(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let offset = frame.read_i16();

    let value2 = frame.operand_stack.pop().int();
    let value1 = frame.operand_stack.pop().int();

    if value1.0 == value2.0 {
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

pub fn if_int_cmp_gt(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let offset = frame.read_i16();

    let value2 = frame.operand_stack.pop().int();
    let value1 = frame.operand_stack.pop().int();

    if value1.0 > value2.0 {
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

pub fn if_ref_cmp_eq(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let offset = frame.read_i16();

    let value2 = frame.operand_stack.pop().reference();
    let value1 = frame.operand_stack.pop().reference();

    if value1.0 == value2.0 {
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

pub fn if_le(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let offset = frame.read_i16();
    let value = frame.operand_stack.pop().int();

    if value.0 <= 0 {
        let mut pc = frame.pc as i64;
        pc -= 3;
        pc += offset as i64;
        frame.pc = pc as usize;
    }
}

pub fn if_gt(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let offset = frame.read_i16();
    let value = frame.operand_stack.pop().int();

    if value.0 > 0 {
        let mut pc = frame.pc as i64;
        pc -= 3;
        pc += offset as i64;
        frame.pc = pc as usize;
    }
}

pub fn if_ge(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    let offset = frame.read_i16();
    let value = frame.operand_stack.pop().int();

    if value.0 >= 0 {
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

pub fn fcmp(thread: &mut Thread, nan: i32) {
    let frame = thread.stack.last_mut().unwrap();
    let value2 = frame.operand_stack.pop().float().0;
    let value1 = frame.operand_stack.pop().float().0;

    let result = if value1 > value2 {
        1
    } else if value1 == value2 {
        0
    } else if value1 < value2 {
        -1
    } else {
        nan
    };

    frame.operand_stack.push(Value::Int(Int(result)));
}

pub fn lcmp(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let value2 = frame.operand_stack.pop().long().0;
    let value1 = frame.operand_stack.pop().long().0;

    let result = if value1 > value2 {
        1
    } else if value1 == value2 {
        0
    } else {
        -1
    };

    frame.operand_stack.push(Value::Int(Int(result)));
}

pub fn lookup_switch(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let start_pc = frame.pc - 1;

    let offset = frame.pc % 4;
    let padding_required = (4 - offset) % 4;
    for _ in 0..padding_required {
        frame.read_u8();
    }

    let default = frame.read_i32();
    let n_pairs = frame.read_i32();
    let mut pairs = vec![];
    for _ in 0..n_pairs {
        pairs.push((frame.read_i32(), frame.read_i32()));
    }

    let key = frame.operand_stack.pop().int().0;
    let offset = pairs.iter().find(|(k,_)| *k == key).map(|(_,off)| *off).unwrap_or_else(|| default);

    let mut pc = start_pc as i64;
    pc += offset as i64;
    let pc = pc as usize;
    frame.pc = pc;
}
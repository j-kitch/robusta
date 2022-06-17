use crate::thread::Thread;

pub fn wide(thread: &mut Thread) {
    let mut current = thread.frames.current_mut();
    let op_code = current.read_u8();
    let index = current.read_u16();

    match op_code {
        0x15 => current.op_stack.push_int(current.local_vars.load_int(index)),
        0x16 => current.op_stack.push_long(current.local_vars.load_long(index)),
        0x17 => current.op_stack.push_float(current.local_vars.load_float(index)),
        0x18 => current.op_stack.push_double(current.local_vars.load_double(index)),
        0x19 => current.op_stack.push_ref(current.local_vars.load_ref(index)),
        0x36 => current.local_vars.store_int(index, current.op_stack.pop_int()),
        0x37 => current.local_vars.store_long(index, current.op_stack.pop_long()),
        0x38 => current.local_vars.store_float(index, current.op_stack.pop_float()),
        0x39 => current.local_vars.store_double(index, current.op_stack.pop_double()),
        0x3A => current.local_vars.store_ref(index, current.op_stack.pop_ref()),
        0x84 => {
            let mut int = current.local_vars.load_int(index);
            let cons = current.read_i16() as i32;
            int += cons;
            current.local_vars.store_int(index, int);
        }
        0xA9 => {
            current.pc = current.local_vars.load_return_address(index);
        }
        _ => panic!("wide does not support op {}", op_code)
    }
}

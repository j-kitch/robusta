use crate::thread::Thread;

pub fn dup(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value = current.op_stack.pop_int();
    current.op_stack.push_int(value);
    current.op_stack.push_int(value);
}

pub fn dup_x1(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value1 = current.op_stack.pop_int();
    let value2 = current.op_stack.pop_int();

    current.op_stack.push_int(value1);
    current.op_stack.push_int(value2);
    current.op_stack.push_int(value1);
}

pub fn dup_x2(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value1 = current.op_stack.pop_int();
    let value2 = current.op_stack.pop_long();

    current.op_stack.push_int(value1);
    current.op_stack.push_long(value2);
    current.op_stack.push_int(value1);
}

pub fn dup2(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value = current.op_stack.pop_long();
    current.op_stack.push_long(value);
    current.op_stack.push_long(value);
}

pub fn dup2_x1(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value1 = current.op_stack.pop_long();
    let value2 = current.op_stack.pop_int();

    current.op_stack.push_long(value1);
    current.op_stack.push_int(value2);
    current.op_stack.push_long(value1);
}

pub fn dup2_x2(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value1 = current.op_stack.pop_long();
    let value2 = current.op_stack.pop_long();

    current.op_stack.push_long(value1);
    current.op_stack.push_long(value2);
    current.op_stack.push_long(value1);
}

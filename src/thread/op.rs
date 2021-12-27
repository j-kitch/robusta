use crate::thread::{Frame, Thread};

type Op = fn(&mut Thread);

pub fn get_op(frame: &mut Frame, code: u8) -> Op {
    match code {
        0xB1 => return_op,
        0x2A => |t| aload_n(t, 0),
        0x2B => |t| aload_n(t, 1),
        0x2C => |t| aload_n(t, 2),
        0x2D => |t| aload_n(t, 3),
        _ => panic!("Unknown op at {}.{}{} PC {} {:#02x}",
                    &frame.class.this_class,
                    &frame.method.name,
                    &frame.method.descriptor,
                    frame.pc - 1,
                    code)
    }
}

fn return_op(thread: &mut Thread) {
    thread.frames.pop();
}

fn aload_n(thread: &mut Thread, n: u16) {
    let mut curr = thread.curr();
    let local_ref = curr.local_vars.load_ref(n);
    curr.op_stack.push_ref(local_ref);
}

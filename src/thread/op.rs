use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;
use crate::heap::Array;
use crate::thread::{Frame, Thread};

type Op = fn(&mut Thread);

pub fn get_op(frame: &mut Frame, code: u8) -> Op {
    match code {
        0xB1 => return_op,
        0x2A => |t| aload_n(t, 0),
        0x2B => |t| aload_n(t, 1),
        0x2C => |t| aload_n(t, 2),
        0x2D => |t| aload_n(t, 3),
        0x4B => |t| astore_n(t, 0),
        0x4C => |t| astore_n(t, 1),
        0x4D => |t| astore_n(t, 2),
        0x4E => |t| astore_n(t, 3),
        0xBE => array_length,
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

fn astore_n(thread: &mut Thread, n: u16) {
    let mut curr = thread.curr();
    let stack_ref = curr.op_stack.pop_ref();
    curr.local_vars.store_ref(n, stack_ref);
}

fn array_length(thread: &mut Thread) {
    let array_ref = thread.pop_ref();
    let arr = thread.object(array_ref);
    let arr = arr.as_ref().borrow();
    let arr = arr.deref();

    let arr: &Array = match arr {
        crate::heap::Ref::Arr(arr) => arr,
        _ => panic!("err")
    };

    let arr_len = arr.len();

    thread.push_int(arr_len);
}

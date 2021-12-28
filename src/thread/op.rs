use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;

use crate::heap::Array;
use crate::thread::{Frame, Thread};

type Op = fn(&mut Thread);

pub fn get_op(frame: &mut Frame, code: u8) -> Op {
    match code {
        0x02 => |t| iconst_n(t, -1),
        0x03 => |t| iconst_n(t, 0),
        0x04 => |t| iconst_n(t, 1),
        0x05 => |t| iconst_n(t, 2),
        0x06 => |t| iconst_n(t, 3),
        0x07 => |t| iconst_n(t, 4),
        0x08 => |t| iconst_n(t, 5),
        0x19 => aload,
        0x1A => |t| iload_n(t, 0),
        0x1B => |t| iload_n(t, 1),
        0x1C => |t| iload_n(t, 2),
        0x1D => |t| iload_n(t, 3),
        0xB1 => return_op,
        0x2A => |t| aload_n(t, 0),
        0x2B => |t| aload_n(t, 1),
        0x2C => |t| aload_n(t, 2),
        0x2D => |t| aload_n(t, 3),
        0x32 => aa_load,
        0x3A => astore,
        0x3B => |t| istore_n(t, 0),
        0x3C => |t| istore_n(t, 1),
        0x3D => |t| istore_n(t, 2),
        0x3E => |t| istore_n(t, 3),
        0x4B => |t| astore_n(t, 0),
        0x4C => |t| astore_n(t, 1),
        0x4D => |t| astore_n(t, 2),
        0x4E => |t| astore_n(t, 3),
        0x9F => |t| if_icmp_cond(t, |i1, i2| i1 == i2),
        0xA0 => |t| if_icmp_cond(t, |i1, i2| i1 != i2),
        0xA1 => |t| if_icmp_cond(t, |i1, i2| i1 < i2),
        0xA2 => |t| if_icmp_cond(t, |i1, i2| i1 >= i2),
        0xA3 => |t| if_icmp_cond(t, |i1, i2| i1 > i2),
        0xA4 => |t| if_icmp_cond(t, |i1, i2| i1 <= i2),
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

fn astore(thread: &mut Thread) {
    let idx = thread.curr().read_u8() as u16;
    let object_ref = thread.pop_ref();
    thread.curr().local_vars.store_ref(idx, object_ref);
}

fn aload(thread: &mut Thread) {
    let idx = thread.curr().read_u8() as u16;
    let object_ref = thread.curr().local_vars.load_ref(idx);
    thread.push_ref(object_ref);
}

fn astore_n(thread: &mut Thread, n: u16) {
    let mut curr = thread.curr();
    let stack_ref = curr.op_stack.pop_ref();
    curr.local_vars.store_ref(n, stack_ref);
}

fn iconst_n(thread: &mut Thread, n: i32) {
    let mut curr = thread.curr();
    curr.op_stack.push_int(n);
}

fn istore_n(thread: &mut Thread, n: u16) {
    let mut curr = thread.curr();
    let stack_int = curr.op_stack.pop_int();
    curr.local_vars.store_int(n, stack_int);
}

fn iload_n(thread: &mut Thread, n: u16) {
    let mut curr = thread.curr();
    let local_int = curr.local_vars.load_int(n);
    curr.op_stack.push_int(local_int);
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

fn if_icmp_cond<F>(thread: &mut Thread, pred: F) where F: Fn(i32, i32) -> bool {
    let mut curr = thread.curr();
    let signed_off = curr.read_i16();
    let start_pc = curr.pc - 3;

    let value2 = curr.op_stack.pop_int();
    let value1 = curr.op_stack.pop_int();

    if pred(value1, value2) {
        let mut pc: i64 = start_pc as i64;
        pc += signed_off as i64;
        curr.pc = pc as u32;
    }
}

fn aa_load(thread: &mut Thread) {
    let idx = thread.pop_int();
    let arr_ref = thread.pop_ref();

    let arr = thread.object(arr_ref);
    let arr = arr.as_ref().borrow();
    let arr = arr.deref();
    let arr: &Array = match arr {
        crate::heap::Ref::Arr(arr) => arr,
        _ => panic!("err")
    };
    let arr = match arr {
        crate::heap::Array::Ref(arr) => arr,
        _ => panic!("err")
    };

    let arr_val = arr[idx as usize];

    thread.push_ref(arr_val);
}

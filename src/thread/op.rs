use std::ops::Deref;

use crate::heap::Array;
use crate::thread::{Frame, Thread};
use crate::thread::local_vars::{Locals, LocalVars};
use crate::thread::op_stack::{OperandStack, OpStack};

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
        0x84 => iinc,
        0x9F => |t| if_icmp_cond(t, |i1, i2| i1 == i2),
        0xA0 => |t| if_icmp_cond(t, |i1, i2| i1 != i2),
        0xA1 => |t| if_icmp_cond(t, |i1, i2| i1 < i2),
        0xA2 => |t| if_icmp_cond(t, |i1, i2| i1 >= i2),
        0xA3 => |t| if_icmp_cond(t, |i1, i2| i1 > i2),
        0xA4 => |t| if_icmp_cond(t, |i1, i2| i1 <= i2),
        0xA7 => goto,
        0xB8 => invoke_static,
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
    let local_ref = thread.load_ref(n);
    thread.push_ref(local_ref);
}

fn astore(thread: &mut Thread) {
    let idx = thread.frame_mut().read_u8() as u16;
    let object_ref = thread.pop_ref();
    thread.store_ref(idx, object_ref);
}

fn aload(thread: &mut Thread) {
    let idx = thread.frame_mut().read_u8() as u16;
    let object_ref = thread.load_ref(idx);
    thread.push_ref(object_ref);
}

fn astore_n(thread: &mut Thread, n: u16) {
    let stack_ref = thread.pop_ref();
    thread.store_ref(n, stack_ref);
}

fn iconst_n(thread: &mut Thread, n: i32) {
    thread.push_int(n);
}

fn istore_n(thread: &mut Thread, n: u16) {
    let stack_int = thread.pop_int();
    thread.store_int(n, stack_int);
}

fn iload_n(thread: &mut Thread, n: u16) {
    let local_int = thread.load_int(n);
    thread.push_int(local_int);
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
    let signed_off = thread.read_i16();
    let start_pc = thread.frame().pc - 3;

    let value2 = thread.pop_int();
    let value1 = thread.pop_int();

    if pred(value1, value2) {
        let mut pc: i64 = start_pc as i64;
        pc += signed_off as i64;
        thread.frame_mut().pc = pc as u32;
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

fn invoke_static(thread: &mut Thread) {
    let method_idx = thread.read_u16();
    let method_ref = match thread.frame_mut().class.const_pool.get(&method_idx).unwrap() {
        crate::class::Const::Method(ref method_ref) => method_ref.clone(),
        _ => panic!("err")
    };
    let class = thread.load(&method_ref.class).unwrap();
    let method = class.find_method(&method_ref.name, &method_ref.descriptor).unwrap();

    let n_args = method.descriptor.category();
    let mut args: Vec<u32> = (0..n_args).map(|_| thread.pop_ref()).collect();
    args.reverse();

    if method.native {
        let mut local_vars = LocalVars::new(args.len() as u16);
        for (idx, word) in args.iter().enumerate() {
            local_vars.store_ref(idx as u16, word.clone());
        }
        let rt = thread.rt.clone();
        let rt = rt.deref().borrow();
        let func = rt.deref().native.find_method(&method_ref.class, &method_ref.name, &method_ref.descriptor);
        func(thread, local_vars);
        // TODO: Assuming no returned value atm.
    } else {
        let mut local_vars = LocalVars::new(method.max_locals.clone());
        for (idx, word) in args.iter().enumerate() {
            local_vars.store_ref(idx as u16, word.clone());
        }
        let frame = Frame {
            pc: 0,
            class: class.clone(),
            local_vars,
            op_stack: OperandStack::new(method.max_stack.clone()),
            method: method,
        };

        thread.frames.push(frame);
    }
}

fn iinc(thread: &mut Thread) {
    let idx = thread.read_u8();
    let inc = thread.read_i8() as i32;
    let int = thread.load_int(idx as u16);
    let (result, _) = int.overflowing_add(inc);
    thread.store_int(idx as u16, result)
}

fn goto(thread: &mut Thread) {
    let off = thread.read_i16();
    let start_pc = thread.frame_mut().pc as i64 - 3;
    let result = start_pc + off as i64;
    thread.frame_mut().pc = result as u32;
}

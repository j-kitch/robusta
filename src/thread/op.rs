use std::ops::Deref;

use crate::heap::Ref;
use crate::thread::{Frame, Thread};
use crate::thread::local_vars::LocalVars;
use crate::thread::op_stack::OperandStack;

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
    let current = thread.frames.current_mut();
    let reference = current.local_vars.load_ref(n);
    current.op_stack.push_ref(reference);
}

fn astore(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let local_var_idx = current.read_u8() as u16;
    let reference = current.op_stack.pop_ref();
    current.local_vars.store_ref(local_var_idx, reference);
}

fn aload(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let local_var_idx = current.read_u8() as u16;
    let reference = current.local_vars.load_ref(local_var_idx);
    current.op_stack.push_ref(reference);
}

fn astore_n(thread: &mut Thread, n: u16) {
    let current = thread.frames.current_mut();
    let reference = current.op_stack.pop_ref();
    current.local_vars.store_ref(n, reference);
}

fn iconst_n(thread: &mut Thread, n: i32) {
    let current = thread.frames.current_mut();
    current.op_stack.push_int(n);
}

fn istore_n(thread: &mut Thread, n: u16) {
    let current = thread.frames.current_mut();
    let int = current.op_stack.pop_int();
    current.local_vars.store_int(n, int);
}

fn iload_n(thread: &mut Thread, n: u16) {
    let current = thread.frames.current_mut();
    let int = current.local_vars.load_int(n);
    current.op_stack.push_int(int);
}

fn array_length(thread: &mut Thread) {
    let runtime = thread.rt.as_ref().borrow();
    let current = thread.frames.current_mut();
    let array_ref = current.op_stack.pop_ref();
    let array_obj = runtime.load_object(array_ref);
    let array_obj = array_obj.deref().borrow();
    let array = match array_obj.deref() {
        Ref::Arr(arr) => arr,
        _ => panic!("err)")
    };

    let array_len = array.len();

    current.op_stack.push_int(array_len);
}

fn if_icmp_cond<F>(thread: &mut Thread, pred: F) where F: Fn(i32, i32) -> bool {
    let current = thread.frames.current_mut();
    let pc_offset = current.read_i16();
    let value2 = current.op_stack.pop_int();
    let value1 = current.op_stack.pop_int();

    if pred(value1, value2) {
        let mut signed_pc: i64 = current.pc as i64;
        signed_pc -= 3;
        signed_pc += pc_offset as i64;
        current.pc = signed_pc as u32;
    }
}

fn aa_load(thread: &mut Thread) {
    let runtime = thread.rt.as_ref().borrow();
    let current = thread.frames.current_mut();
    let elem_idx = current.op_stack.pop_int();
    let array_ref = current.op_stack.pop_ref();
    let array_obj = runtime.load_object(array_ref);
    let array_obj = array_obj.as_ref().borrow();
    let array = match array_obj.deref() {
        Ref::Arr(arr) => arr,
        _ => panic!("err)")
    };
    let array = match array {
        crate::heap::Array::Ref(array) => array,
        _ => panic!("err")
    };

    let array_value = array[elem_idx as usize];

    current.op_stack.push_ref(array_value);
}

fn invoke_static(thread: &mut Thread) {
    let runtime = thread.rt.clone();
    let mut runtime = runtime.borrow_mut();
    let current = thread.frames.current_mut();

    let method_idx = current.read_u16();
    let method_ref = current.class.const_method(method_idx);
    let class = runtime.load_class(&method_ref.class);
    let method = class.find_method(&method_ref.name, &method_ref.descriptor).unwrap();

    let n_args = method.descriptor.category();
    let mut args: Vec<u32> = (0..n_args).map(|_| current.op_stack.pop_ref()).collect();
    args.reverse();

    if method.native {
        let mut local_vars = LocalVars::new(args.len() as u16);
        for (idx, word) in args.iter().enumerate() {
            local_vars.store_ref(idx as u16, word.clone());
        }
        let func = runtime.native.find_method(&method_ref.class, &method_ref.name, &method_ref.descriptor);
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
            method,
        };

        thread.frames.push(frame);
    }
}

fn iinc(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let idx = current.read_u8();
    let inc = current.read_i8() as i32;
    let int = current.local_vars.load_int(idx as u16);
    let (result, _) = int.overflowing_add(inc);
    current.local_vars.store_int(idx as u16, result)
}

fn goto(thread: &mut Thread) {
    let mut current = thread.frames.current_mut();
    let off = current.read_i16();
    let start_pc = current.pc as i64 - 3;
    let result = start_pc + off as i64;
    current.pc = result as u32;
}

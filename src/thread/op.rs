use std::ops::{Deref, DerefMut};
use crate::class::Const;

use crate::descriptor::Descriptor;
use crate::heap::Value;
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
        0x12 => ldc,
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
        0x60 => iadd,
        0x64 => isub,
        0x68 => imul,
        0x6C => idiv,
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
    let array = array_obj.arr();

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
    let array = array_obj.arr().reference();

    let array_value = array[elem_idx as usize];

    current.op_stack.push_ref(array_value);
}

fn invoke_static(thread: &mut Thread) {
    let current = thread.frames.current_mut();

    let method_idx = current.read_u16();
    let method_ref = current.class.const_method(method_idx);
    let class = thread.rt.clone().borrow_mut().load_class(&method_ref.class);
    let method = class.find_method(&method_ref.name, &method_ref.descriptor).unwrap();

    let mut args = vec![];
    for arg in method.descriptor.args.iter().rev() {
        match arg {
            Descriptor::Object(_) | Descriptor::Array(_) => {
                args.push(Value::Ref(current.op_stack.pop_ref()));
            }
            Descriptor::Boolean | Descriptor::Byte | Descriptor::Char | Descriptor::Short | Descriptor::Int => {
                args.push(Value::Int(current.op_stack.pop_int()));
            }
            _ => panic!("err")
        }
    }
    args.reverse();

    if method.native {
        let func = thread.rt.as_ref().borrow().native.find_method(&method_ref.class, &method_ref.name, &method_ref.descriptor);
        let mut runtime = thread.rt.as_ref().borrow_mut();
        let result = func(runtime.deref_mut(), args);
        if method.descriptor.returns.is_some() {
            current.op_stack.push(result.unwrap());
        }
    } else {
        thread.create_frame(class.clone(), method, args);
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

fn ldc(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let idx = current.read_u8() as u16;
    let con = current.class.const_pool.get(&idx).unwrap();
    match con {
        Const::Int(i) => {
            current.op_stack.push_int(i.int);
        }
        _ => panic!("err")
    }
}

fn imul(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_int();
    let value1 = current.op_stack.pop_int();

    let (result, _) = value1.overflowing_mul(value2);

    current.op_stack.push_int(result);
}

fn iadd(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_int();
    let value1 = current.op_stack.pop_int();

    let (result, _) = value1.overflowing_add(value2);

    current.op_stack.push_int(result);
}

fn idiv(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_int();
    let value1 = current.op_stack.pop_int();

    let (result, _) = value1.overflowing_div(value2);

    current.op_stack.push_int(result);
}

fn isub(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_int();
    let value1 = current.op_stack.pop_int();

    let (result, _) = value1.overflowing_sub(value2);

    current.op_stack.push_int(result);
}

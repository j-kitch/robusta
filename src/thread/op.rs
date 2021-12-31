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
        0x09 => |t| lconst_n(t, 0),
        0x0A => |t| lconst_n(t, 1),
        0x12 => ldc,
        0x14 => ldc2_w,
        0x19 => aload,
        0x1A => |t| iload_n(t, 0),
        0x1B => |t| iload_n(t, 1),
        0x1C => |t| iload_n(t, 2),
        0x1D => |t| iload_n(t, 3),
        0x1E => |t| lload_n(t, 0),
        0x1F => |t| lload_n(t, 1),
        0x20 => |t| lload_n(t, 2),
        0x21 => |t| lload_n(t, 3),
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
        0x3F => |t| lstore_n(t, 0),
        0x40 => |t| lstore_n(t, 1),
        0x41 => |t| lstore_n(t, 2),
        0x42 => |t| lstore_n(t, 3),
        0x4B => |t| astore_n(t, 0),
        0x4C => |t| astore_n(t, 1),
        0x4D => |t| astore_n(t, 2),
        0x4E => |t| astore_n(t, 3),
        0x5C => dup2,
        0x60 => |t| int_binary_op(t, |i1, i2| i1.overflowing_add(i2).0),
        0x61 => |t| long_binary_op(t, |l1, l2| l1.overflowing_add(l2).0),
        0x64 => |t| int_binary_op(t, |i1, i2| i1.overflowing_sub(i2).0),
        0x65 => |t| long_binary_op(t, |l1, l2| l1.overflowing_sub(l2).0),
        0x68 => |t| int_binary_op(t, |i1, i2| i1.overflowing_mul(i2).0),
        0x69 => |t| long_binary_op(t, |l1, l2| l1.overflowing_mul(l2).0),
        0x6C => |t| int_binary_op(t, |i1, i2| i1.overflowing_div(i2).0),
        0x6D => |t| long_binary_op(t, |l1, l2| l1.overflowing_div(l2).0),
        0x70 => |t| int_binary_op(t, |i1, i2| i1.overflowing_rem(i2).0),
        0x71 => |t| long_binary_op(t, |l1, l2| l1.overflowing_rem(l2).0),
        0x74 => ineg,
        0x75 => lneg,
        0x78 => ishl,
        0x79 => lshl,
        0x7A => ishr,
        0x7B => lshr,
        0x7C => iushr,
        0x7D => lushr,
        0x7E => |t| int_binary_op(t, |i1, i2| i1 & i2),
        0x7F => |t| long_binary_op(t, |l1, l2| l1 & l2),
        0x80 => |t| int_binary_op(t, |i1, i2| i1 | i2),
        0x81 => |t| long_binary_op(t, |l1, l2| l1 | l2),
        0x82 => |t| int_binary_op(t, |i1, i2| i1 ^ i2),
        0x83 => |t| long_binary_op(t, |l1, l2| l1 ^ l2),
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
        0xCA => reserved,
        0xFE => reserved,
        0xFF => reserved,
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

fn dup2(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let eight_byte_op = current.op_stack.pop_long();
    current.op_stack.push_long(eight_byte_op);
    current.op_stack.push_long(eight_byte_op);
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

fn lconst_n(thread: &mut Thread, n: i64) {
    let current = thread.frames.current_mut();
    current.op_stack.push_long(n);
}

fn istore_n(thread: &mut Thread, n: u16) {
    let current = thread.frames.current_mut();
    let int = current.op_stack.pop_int();
    current.local_vars.store_int(n, int);
}

fn lstore_n(thread: &mut Thread, n: u16) {
    let current = thread.frames.current_mut();
    let long = current.op_stack.pop_long();
    current.local_vars.store_long(n, long);
}

fn iload_n(thread: &mut Thread, n: u16) {
    let current = thread.frames.current_mut();
    let int = current.local_vars.load_int(n);
    current.op_stack.push_int(int);
}

fn lload_n(thread: &mut Thread, n: u16) {
    let current = thread.frames.current_mut();
    let long = current.local_vars.load_long(n);
    current.op_stack.push_long(long);
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
            Descriptor::Long => {
                args.push(Value::Long(current.op_stack.pop_long()))
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

fn ldc2_w(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let idx = current.read_u16();
    let con = current.class.const_pool.get(&idx).unwrap();
    match con {
        Const::Long(l) => {
            current.op_stack.push_long(l.long);
        }
        _ => panic!("err")
    }
}

fn int_binary_op<F>(thread: &mut Thread, op: F) where F: Fn(i32, i32) -> i32 {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_int();
    let value1 = current.op_stack.pop_int();

    let result = op(value1, value2);

    current.op_stack.push_int(result);
}

fn long_binary_op<F>(thread: &mut Thread, op: F) where F: Fn(i64, i64) -> i64 {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_long();
    let value1 = current.op_stack.pop_long();

    let result = op(value1, value2);

    current.op_stack.push_long(result);
}

fn ineg(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value1 = current.op_stack.pop_int();

    let result = -value1;

    current.op_stack.push_int(result);
}

fn lneg(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value1 = current.op_stack.pop_long();

    let result = -value1;

    current.op_stack.push_long(result);
}

fn ishl(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_int();
    let value1 = current.op_stack.pop_int();

    let s = (0x1F & value2) as u32;

    let (result, _) = value1.overflowing_shl(s);

    current.op_stack.push_int(result);
}

fn lshl(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_int();
    let value1 = current.op_stack.pop_long();

    let s = (0x1F & value2) as u32;

    let (result, _) = value1.overflowing_shl(s);

    current.op_stack.push_long(result);
}

fn ishr(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_int();
    let value1 = current.op_stack.pop_int();

    let s = (0x1F & value2) as u32;

    let (result, _) = value1.overflowing_shr(s);

    current.op_stack.push_int(result);
}

fn lshr(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_int();
    let value1 = current.op_stack.pop_long();

    let s = (0x1F & value2) as u32;

    let (result, _) = value1.overflowing_shr(s);

    current.op_stack.push_long(result);
}

fn iushr(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_int();
    let value1 = current.op_stack.pop_int();

    let s = (0x1F & value2) as u32;
    let uns_value1 = u32::from_be_bytes(value1.to_be_bytes());

    let (result, _) = uns_value1.overflowing_shr(s);
    let result = i32::from_be_bytes(result.to_be_bytes());

    current.op_stack.push_int(result);
}

fn lushr(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_int();
    let value1 = current.op_stack.pop_long();

    let s = (0x1F & value2) as u32;
    let uns_value1 = u64::from_be_bytes(value1.to_be_bytes());

    let (result, _) = uns_value1.overflowing_shr(s);
    let result = i64::from_be_bytes(result.to_be_bytes());

    current.op_stack.push_long(result);
}

fn reserved(thread: &mut Thread) {
    let current = thread.frames.current();
    panic!("encountered reserved opcode {} at {}.{}{}",
        current.method.code[(current.pc - 1) as usize],
        &current.class.this_class,
        &current.method.name,
        current.method.descriptor.descriptor());
}

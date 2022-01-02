use std::ops::{Deref, DerefMut};
use crate::class::Const;

use crate::descriptor::Descriptor;
use crate::heap;
use crate::heap::Value;
use crate::instruction::array_load;
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
        0x10 => bipush,
        0x11 => sipush,
        0x0A => |t| lconst_n(t, 1),
        0x0B => |t| fconst_n(t, 0.),
        0x0C => |t| fconst_n(t, 1.),
        0x0D => |t| fconst_n(t, 2.),
        0x0E => |t| dconst_n(t, 0.),
        0x0F => |t| dconst_n(t, 1.),
        0x12 => ldc,
        0x14 => ldc2_w,
        0x17 => fload,
        0x18 => dload,
        0x19 => aload,
        0x1A => |t| iload_n(t, 0),
        0x1B => |t| iload_n(t, 1),
        0x1C => |t| iload_n(t, 2),
        0x1D => |t| iload_n(t, 3),
        0x1E => |t| lload_n(t, 0),
        0x1F => |t| lload_n(t, 1),
        0x20 => |t| lload_n(t, 2),
        0x21 => |t| lload_n(t, 3),
        0x22 => |t| fload_n(t, 0),
        0x23 => |t| fload_n(t, 1),
        0x24 => |t| fload_n(t, 2),
        0x25 => |t| fload_n(t, 3),
        0x26 => |t| dload_n(t, 0),
        0x27 => |t| dload_n(t, 1),
        0x28 => |t| dload_n(t, 2),
        0x29 => |t| dload_n(t, 3),
        0x2A => |t| aload_n(t, 0),
        0x2B => |t| aload_n(t, 1),
        0x2C => |t| aload_n(t, 2),
        0x2D => |t| aload_n(t, 3),
        0x2E => array_load::int,
        0x2F => array_load::long,
        0x30 => array_load::float,
        0x31 => array_load::double,
        0x32 => array_load::reference,
        0x33 => array_load::byte,
        0x34 => array_load::char,
        0x35 => array_load::short,
        0x38 => fstore,
        0x39 => dstore,
        0x3A => astore,
        0x3B => |t| istore_n(t, 0),
        0x3C => |t| istore_n(t, 1),
        0x3D => |t| istore_n(t, 2),
        0x3E => |t| istore_n(t, 3),
        0x3F => |t| lstore_n(t, 0),
        0x40 => |t| lstore_n(t, 1),
        0x41 => |t| lstore_n(t, 2),
        0x42 => |t| lstore_n(t, 3),
        0x43 => |t| fstore_n(t, 0),
        0x44 => |t| fstore_n(t, 1),
        0x45 => |t| fstore_n(t, 2),
        0x46 => |t| fstore_n(t, 3),
        0x47 => |t| dstore_n(t, 0),
        0x48 => |t| dstore_n(t, 1),
        0x49 => |t| dstore_n(t, 2),
        0x4A => |t| dstore_n(t, 3),
        0x4B => |t| astore_n(t, 0),
        0x4C => |t| astore_n(t, 1),
        0x4D => |t| astore_n(t, 2),
        0x4E => |t| astore_n(t, 3),
        0x4F => iastore,
        0x50 => lastore,
        0x51 => fastore,
        0x52 => dastore,
        0x54 => bastore,
        0x55 => castore,
        0x56 => sastore,
        0x59 => dup,
        0x5C => dup2,
        0x60 => |t| int_binary_op(t, |i1, i2| i1.overflowing_add(i2).0),
        0x61 => |t| long_binary_op(t, |l1, l2| l1.overflowing_add(l2).0),
        0x62 => |t| float_binary_op(t, |f1, f2| f1 + f2),
        0x63 => |t| double_binary_op(t, |d1, d2| d1 + d2),
        0x64 => |t| int_binary_op(t, |i1, i2| i1.overflowing_sub(i2).0),
        0x65 => |t| long_binary_op(t, |l1, l2| l1.overflowing_sub(l2).0),
        0x66 => |t| float_binary_op(t, |f1, f2| f1 - f2),
        0x67 => |t| double_binary_op(t, |d1, d2| d1 - d2),
        0x68 => |t| int_binary_op(t, |i1, i2| i1.overflowing_mul(i2).0),
        0x69 => |t| long_binary_op(t, |l1, l2| l1.overflowing_mul(l2).0),
        0x6A => |t| float_binary_op(t, |f1, f2| f1 * f2),
        0x6B => |t| double_binary_op(t, |d1, d2| d1 * d2),
        0x6C => |t| int_binary_op(t, |i1, i2| i1.overflowing_div(i2).0),
        0x6D => |t| long_binary_op(t, |l1, l2| l1.overflowing_div(l2).0),
        0x6E => |t| float_binary_op(t, |f1, f2| f1 / f2),
        0x6F => |t| double_binary_op(t, |d1, d2| d1 / d2),
        0x70 => |t| int_binary_op(t, |i1, i2| i1.overflowing_rem(i2).0),
        0x71 => |t| long_binary_op(t, |l1, l2| l1.overflowing_rem(l2).0),
        0x72 => |t| float_binary_op(t, |f1, f2| f1 % f2),
        0x73 => |t| double_binary_op(t, |d1, d2| d1 % d2),
        0x74 => ineg,
        0x75 => lneg,
        0x76 => fneg,
        0x77 => dneg,
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
        0x85 => i2l,
        0x86 => i2f,
        0x87 => i2d,
        0x88 => l2i,
        0x89 => l2f,
        0x8A => l2d,
        0x8B => f2i,
        0x8C => f2l,
        0x8D => f2d,
        0x8E => d2i,
        0x8F => d2l,
        0x90 => d2f,
        0x91 => i2b,
        0x92 => i2c,
        0x93 => i2s,
        0x9F => |t| if_icmp_cond(t, |i1, i2| i1 == i2),
        0xA0 => |t| if_icmp_cond(t, |i1, i2| i1 != i2),
        0xA1 => |t| if_icmp_cond(t, |i1, i2| i1 < i2),
        0xA2 => |t| if_icmp_cond(t, |i1, i2| i1 >= i2),
        0xA3 => |t| if_icmp_cond(t, |i1, i2| i1 > i2),
        0xA4 => |t| if_icmp_cond(t, |i1, i2| i1 <= i2),
        0xA7 => goto,
        0xB1 => return_op,
        0xB8 => invoke_static,
        0xBC => new_array,
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

fn dup(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let four_byte_word = current.op_stack.pop_ref();
    current.op_stack.push_ref(four_byte_word);
    current.op_stack.push_ref(four_byte_word);
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

fn dload(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let idx = current.read_u8() as u16;
    dload_n(thread, idx);
}

fn dload_n(thread: &mut Thread, n: u16) {
    let current = thread.frames.current_mut();
    let double = current.local_vars.load_double(n);
    current.op_stack.push_double(double);
}

fn astore_n(thread: &mut Thread, n: u16) {
    let current = thread.frames.current_mut();
    let reference = current.op_stack.pop_ref();
    current.local_vars.store_ref(n, reference);
}

fn dstore_n(thread: &mut Thread, n: u16) {
    let current = thread.frames.current_mut();
    let double = current.op_stack.pop_double();
    current.local_vars.store_double(n, double);
}

fn dstore(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let idx = current.read_u8() as u16;
    dstore_n(thread, idx)
}

fn dconst_n(thread: &mut Thread, n: f64) {
    let current = thread.frames.current_mut();
    current.op_stack.push_double(n)
}

fn iconst_n(thread: &mut Thread, n: i32) {
    let current = thread.frames.current_mut();
    current.op_stack.push_int(n);
}

fn fconst_n(thread: &mut Thread, n: f32) {
    let current = thread.frames.current_mut();
    current.op_stack.push_float(n);
}

fn lconst_n(thread: &mut Thread, n: i64) {
    let current = thread.frames.current_mut();
    current.op_stack.push_long(n);
}

fn fstore_n(thread: &mut Thread, n: u16) {
    let current = thread.frames.current_mut();
    let float = current.op_stack.pop_float();
    current.local_vars.store_float(n, float);
}

fn fstore(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let idx = current.read_u8() as u16;
    fstore_n(thread, idx)
}

fn fload(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let idx = current.read_u8() as u16;
    fload_n(thread, idx)
}

fn fload_n(thread: &mut Thread, n: u16) {
    let current = thread.frames.current_mut();
    let float = current.local_vars.load_float(n);
    current.op_stack.push_float(float)
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
            Descriptor::Float => {
                args.push(Value::Float(current.op_stack.pop_float()))
            }
            Descriptor::Long => {
                args.push(Value::Long(current.op_stack.pop_long()))
            }
            Descriptor::Double => {
                args.push(Value::Double(current.op_stack.pop_double()))
            }
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
        Const::Float(f) => {
            current.op_stack.push_float(f.float);
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
        Const::Double(d) => {
            current.op_stack.push_double(d.double);
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

fn fneg(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value1 = current.op_stack.pop_float();

    let result = -value1;

    current.op_stack.push_float(result);
}

fn dneg(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value1 = current.op_stack.pop_double();

    let result = -value1;

    current.op_stack.push_double(result);
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

fn float_binary_op<F>(thread: &mut Thread, op: F) where F: Fn(f32, f32) -> f32 {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_float();
    let value1 = current.op_stack.pop_float();

    let result = op(value1, value2);

    current.op_stack.push_float(result);
}

fn double_binary_op<F>(thread: &mut Thread, op: F) where F: Fn(f64, f64) -> f64 {
    let current = thread.frames.current_mut();
    let value2 = current.op_stack.pop_double();
    let value1 = current.op_stack.pop_double();

    let result = op(value1, value2);

    current.op_stack.push_double(result);
}

fn reserved(thread: &mut Thread) {
    let current = thread.frames.current();
    panic!("encountered reserved opcode {} at {}.{}{}",
        current.method.code[(current.pc - 1) as usize],
        &current.class.this_class,
        &current.method.name,
        current.method.descriptor.descriptor());
}

fn i2b(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value = current.op_stack.pop_int();

    let result = value & 0xFF;

    current.op_stack.push_int(result);
}

fn i2c(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value = current.op_stack.pop_int();

    let result = value & 0xFF_FF;

    current.op_stack.push_int(result);
}

fn i2d(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value = current.op_stack.pop_int();

    let result = value as f64;

    current.op_stack.push_double(result);
}

fn i2f(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value = current.op_stack.pop_int();

    let result = value as f32;

    current.op_stack.push_float(result);
}

fn i2l(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value = current.op_stack.pop_int();

    let result = value as i64;

    current.op_stack.push_long(result);
}

fn i2s(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value = current.op_stack.pop_int();

    let result = value & 0xFF_FF;

    current.op_stack.push_int(result);
}

fn l2i(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value = current.op_stack.pop_long();

    let bytes: [u8; 8] = value.to_be_bytes();
    let low_order_bytes: [u8; 4] = [bytes[4], bytes[5], bytes[6], bytes[7]];
    let int = i32::from_be_bytes(low_order_bytes);

    current.op_stack.push_int(int);
}

fn l2f(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value = current.op_stack.pop_long();

    let result = value as f32;

    current.op_stack.push_float(result);
}

fn l2d(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value = current.op_stack.pop_long();

    let result = value as f64;

    current.op_stack.push_double(result);
}

fn f2i(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value = current.op_stack.pop_float();

    let result = value as i32;

    current.op_stack.push_int(result);
}

fn f2l(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value = current.op_stack.pop_float();

    let result = value as i64;

    current.op_stack.push_long(result);
}

fn f2d(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value = current.op_stack.pop_float();

    let result = value as f64;

    current.op_stack.push_double(result);
}

fn d2i(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value = current.op_stack.pop_double();

    let result = value as i32;

    current.op_stack.push_int(result);
}

fn d2l(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value = current.op_stack.pop_double();

    let result = value as i64;

    current.op_stack.push_long(result);
}

fn d2f(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let value = current.op_stack.pop_double();

    let result = value as f32;

    current.op_stack.push_float(result);
}

fn new_array(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let arr_type = current.read_u8();
    let count = current.op_stack.pop_int();

    let arr_type = match arr_type {
        4 => Descriptor::Boolean,
        5 => Descriptor::Char,
        6 => Descriptor::Float,
        7 => Descriptor::Double,
        8 => Descriptor::Byte,
        9 => Descriptor::Short,
        10 => Descriptor::Int,
        11 => Descriptor::Long,
        _ => panic!("err"),
    };

    let mut runtime = thread.rt.borrow_mut();
    let array = runtime.heap.create_array(arr_type, count);

    current.op_stack.push_ref(array);
}

fn bastore(thread: &mut Thread) {
    let runtime = thread.rt.borrow();
    let current = thread.frames.current_mut();

    let value = current.op_stack.pop_int() as i8;
    let index = current.op_stack.pop_int() as usize;
    let array_ref = current.op_stack.pop_ref();

    let array = runtime.heap.get(array_ref);
    let mut array = array.as_ref().borrow_mut();
    let array = match array.deref_mut() {
        heap::Ref::Arr(arr) => arr,
        _ => panic!("err"),
    };

    let byte_array = match array {
        heap::Array::Byte(vec) => vec,
        _ => panic!("err"),
    };

    byte_array[index] = value;
}

fn castore(thread: &mut Thread) {
    let runtime = thread.rt.borrow();
    let current = thread.frames.current_mut();

    let value = current.op_stack.pop_int() as u16;
    let index = current.op_stack.pop_int() as usize;
    let array_ref = current.op_stack.pop_ref();

    let array = runtime.heap.get(array_ref);
    let mut array = array.as_ref().borrow_mut();
    let array = match array.deref_mut() {
        heap::Ref::Arr(arr) => arr,
        _ => panic!("err"),
    };

    let char_array = match array {
        heap::Array::Char(vec) => vec,
        _ => panic!("err"),
    };

    char_array[index] = value;
}

fn sastore(thread: &mut Thread) {
    let runtime = thread.rt.borrow();
    let current = thread.frames.current_mut();

    let value = current.op_stack.pop_int() as i16;
    let index = current.op_stack.pop_int() as usize;
    let array_ref = current.op_stack.pop_ref();

    let array = runtime.heap.get(array_ref);
    let mut array = array.as_ref().borrow_mut();
    let array = match array.deref_mut() {
        heap::Ref::Arr(arr) => arr,
        _ => panic!("err"),
    };

    let short_array = match array {
        heap::Array::Short(vec) => vec,
        _ => panic!("err"),
    };

    short_array[index] = value;
}

fn iastore(thread: &mut Thread) {
    let runtime = thread.rt.borrow();
    let current = thread.frames.current_mut();

    let value = current.op_stack.pop_int();
    let index = current.op_stack.pop_int() as usize;
    let array_ref = current.op_stack.pop_ref();

    let array = runtime.heap.get(array_ref);
    let mut array = array.as_ref().borrow_mut();
    let array = match array.deref_mut() {
        heap::Ref::Arr(arr) => arr,
        _ => panic!("err"),
    };

    let int_array = match array {
        heap::Array::Int(vec) => vec,
        _ => panic!("err"),
    };

    int_array[index] = value;
}

fn lastore(thread: &mut Thread) {
    let runtime = thread.rt.borrow();
    let current = thread.frames.current_mut();

    let value = current.op_stack.pop_long();
    let index = current.op_stack.pop_int() as usize;
    let array_ref = current.op_stack.pop_ref();

    let array = runtime.heap.get(array_ref);
    let mut array = array.as_ref().borrow_mut();
    let array = match array.deref_mut() {
        heap::Ref::Arr(arr) => arr,
        _ => panic!("err"),
    };

    let long_array = match array {
        heap::Array::Long(vec) => vec,
        _ => panic!("err"),
    };

    long_array[index] = value;
}

fn bipush(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let byte = current.read_i8() as i32;
    current.op_stack.push_int(byte);
}

fn sipush(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let byte = current.read_i16() as i32;
    current.op_stack.push_int(byte);
}

fn fastore(thread: &mut Thread) {
    let runtime = thread.rt.borrow();
    let current = thread.frames.current_mut();

    let value = current.op_stack.pop_float();
    let index = current.op_stack.pop_int() as usize;
    let array_ref = current.op_stack.pop_ref();

    let array = runtime.heap.get(array_ref);
    let mut array = array.as_ref().borrow_mut();
    let array = match array.deref_mut() {
        heap::Ref::Arr(arr) => arr,
        _ => panic!("err"),
    };

    let float_array = match array {
        heap::Array::Float(vec) => vec,
        _ => panic!("err"),
    };

    float_array[index] = value;
}

fn dastore(thread: &mut Thread) {
    let runtime = thread.rt.borrow();
    let current = thread.frames.current_mut();

    let value = current.op_stack.pop_double();
    let index = current.op_stack.pop_int() as usize;
    let array_ref = current.op_stack.pop_ref();

    let array = runtime.heap.get(array_ref);
    let mut array = array.as_ref().borrow_mut();
    let array = match array.deref_mut() {
        heap::Ref::Arr(arr) => arr,
        _ => panic!("err"),
    };

    let double_array = match array {
        heap::Array::Double(vec) => vec,
        _ => panic!("err"),
    };

    double_array[index] = value;
}

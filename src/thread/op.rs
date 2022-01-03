use std::ops::{Deref, DerefMut};

use crate::descriptor::Descriptor;
use crate::heap::Value;
use crate::instruction::{array_load, array_store, binary_op, convert, dup, load, load_const, pop, push, push_const, shift, single_op, store};
use crate::thread::{Frame, Thread};

type Op = fn(&mut Thread);

pub fn get_op(frame: &mut Frame, code: u8) -> Op {
    match code {
        0x00 => nop,
        0x01 => push_const::reference_null,
        0x02 => push_const::int_m1,
        0x03 => push_const::int_0,
        0x04 => push_const::int_1,
        0x05 => push_const::int_2,
        0x06 => push_const::int_3,
        0x07 => push_const::int_4,
        0x08 => push_const::int_5,
        0x09 => push_const::long_0,
        0x0A => push_const::long_1,
        0x0B => push_const::float_0,
        0x0C => push_const::float_1,
        0x0D => push_const::float_2,
        0x0E => push_const::double_0,
        0x0F => push_const::double_1,
        0x10 => push::byte,
        0x11 => push::short,
        0x12 => load_const::category_1,
        0x13 => load_const::category_1_wide,
        0x14 => load_const::category_2_wide,
        0x15 => load::int,
        0x16 => load::long,
        0x17 => load::float,
        0x18 => load::double,
        0x19 => load::reference,
        0x1A => load::int_0,
        0x1B => load::int_1,
        0x1C => load::int_2,
        0x1D => load::int_3,
        0x1E => load::long_0,
        0x1F => load::long_1,
        0x20 => load::long_2,
        0x21 => load::long_3,
        0x22 => load::float_0,
        0x23 => load::float_1,
        0x24 => load::float_2,
        0x25 => load::float_3,
        0x26 => load::double_0,
        0x27 => load::double_1,
        0x28 => load::double_2,
        0x29 => load::double_3,
        0x2A => load::reference_0,
        0x2B => load::reference_1,
        0x2C => load::reference_2,
        0x2D => load::reference_3,
        0x2E => array_load::int,
        0x2F => array_load::long,
        0x30 => array_load::float,
        0x31 => array_load::double,
        0x32 => array_load::reference,
        0x33 => array_load::byte,
        0x34 => array_load::char,
        0x35 => array_load::short,
        0x36 => store::int,
        0x37 => store::long,
        0x38 => store::float,
        0x39 => store::double,
        0x3A => store::reference,
        0x3B => store::int_0,
        0x3C => store::int_1,
        0x3D => store::int_2,
        0x3E => store::int_3,
        0x3F => store::long_0,
        0x40 => store::long_1,
        0x41 => store::long_2,
        0x42 => store::long_3,
        0x43 => store::float_0,
        0x44 => store::float_1,
        0x45 => store::float_2,
        0x46 => store::float_3,
        0x47 => store::double_0,
        0x48 => store::double_1,
        0x49 => store::double_2,
        0x4A => store::double_3,
        0x4B => store::reference_0,
        0x4C => store::reference_1,
        0x4D => store::reference_2,
        0x4E => store::reference_3,
        0x4F => array_store::int,
        0x50 => array_store::long,
        0x51 => array_store::float,
        0x52 => array_store::double,
        0x54 => array_store::byte,
        0x55 => array_store::char,
        0x56 => array_store::short,
        0x57 => pop::category_1,
        0x58 => pop::category_2,
        0x59 => dup::dup,
        0x5A => dup::dup_x1,
        0x5B => dup::dup_x2,
        0x5C => dup::dup2,
        0x5D => dup::dup2_x1,
        0x5E => dup::dup2_x2,
        0x5F => dup::swap,
        0x60 => binary_op::int_add,
        0x61 => binary_op::long_add,
        0x62 => binary_op::float_add,
        0x63 => binary_op::double_add,
        0x64 => binary_op::int_sub,
        0x65 => binary_op::long_sub,
        0x66 => binary_op::float_sub,
        0x67 => binary_op::double_sub,
        0x68 => binary_op::int_mul,
        0x69 => binary_op::long_mul,
        0x6A => binary_op::float_mul,
        0x6B => binary_op::double_mul,
        0x6C => binary_op::int_div,
        0x6D => binary_op::long_div,
        0x6E => binary_op::float_div,
        0x6F => binary_op::double_div,
        0x70 => binary_op::int_rem,
        0x71 => binary_op::long_rem,
        0x72 => binary_op::float_rem,
        0x73 => binary_op::double_rem,
        0x74 => single_op::int_neg,
        0x75 => single_op::long_neg,
        0x76 => single_op::float_neg,
        0x77 => single_op::double_neg,
        0x78 => shift::int_left,
        0x79 => shift::long_left,
        0x7A => shift::int_right,
        0x7B => shift::long_right,
        0x7C => shift::int_right_unsigned,
        0x7D => shift::long_right_unsigned,
        0x7E => binary_op::int_and,
        0x7F => binary_op::long_and,
        0x80 => binary_op::int_or,
        0x81 => binary_op::long_or,
        0x82 => binary_op::int_xor,
        0x83 => binary_op::long_xor,
        0x84 => single_op::int_inc,
        0x85 => convert::int_to_long,
        0x86 => convert::int_to_float,
        0x87 => convert::int_to_double,
        0x88 => convert::long_to_int,
        0x89 => convert::long_to_float,
        0x8A => convert::long_to_double,
        0x8B => convert::float_to_int,
        0x8C => convert::float_to_long,
        0x8D => convert::float_to_double,
        0x8E => convert::double_to_int,
        0x8F => convert::double_to_long,
        0x90 => convert::double_to_float,
        0x91 => convert::int_to_byte,
        0x92 => convert::int_to_char,
        0x93 => convert::int_to_short,
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

fn nop(_: &mut Thread) {}

fn return_op(thread: &mut Thread) {
    thread.frames.pop();
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

fn goto(thread: &mut Thread) {
    let mut current = thread.frames.current_mut();
    let off = current.read_i16();
    let start_pc = current.pc as i64 - 3;
    let result = start_pc + off as i64;
    current.pc = result as u32;
}

fn reserved(thread: &mut Thread) {
    let current = thread.frames.current();
    panic!("encountered reserved opcode {} at {}.{}{}",
           current.method.code[(current.pc - 1) as usize],
           &current.class.this_class,
           &current.method.name,
           current.method.descriptor.descriptor());
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

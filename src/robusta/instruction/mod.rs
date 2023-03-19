use tracing::trace;
pub use new::new;
use crate::instruction::array::{a_array_load, a_array_store, a_new_array, array_length, char_array_load, char_array_store};
use crate::instruction::branch::{fcmp, goto, if_eq, if_ge, if_gt, if_int_cmp_ge, if_int_cmp_le, if_int_cmp_lt, if_int_cmp_ne, if_le, if_lt, if_ne, if_non_null, if_null, if_ref_cmp_ne};
use crate::instruction::class::{check_cast, instance_of};
use crate::instruction::conv::{float_to_int, int_to_float, int_to_long};
use crate::instruction::dup::{dup, dup_x1};
use crate::instruction::field::{get_field, get_static, put_field, put_static};
use crate::instruction::invoke::{invoke_interface, invoke_special, invoke_static, invoke_virtual};
use crate::instruction::locals::{aload, aload_n, astore, astore_n, fload_n, iload, iload_n, istore, istore_n, lload};
use crate::instruction::math::{f_mul, i_add, i_inc, i_mul, i_sub, iand, irem, iushr, ixor, l_add, land, lshl};
use crate::instruction::new::new_array;
use crate::instruction::r#const::{aconst_null, fconst_n, iconst_n, lconst_n, load_constant, load_constant_cat_2_wide, load_constant_wide};
use crate::instruction::r#return::{a_return, a_throw, d_return, f_return, i_return, r#return};
use crate::instruction::stack::{bipush, pop, sipush};
use crate::instruction::sync::{monitor_enter, monitor_exit};
use crate::log;

use crate::thread::Thread;

mod new;
mod dup;
mod invoke;
mod field;
mod r#return;
mod r#const;
mod array;
mod math;
mod branch;
mod locals;
mod stack;
mod sync;
mod class;
mod conv;

pub fn instruction(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let method = unsafe { frame.method.as_ref().unwrap() };
    let class = unsafe { method.class.as_ref().unwrap() };
    let pc = frame.pc;
    let opcode = frame.read_u8();
    let depth = thread.stack.len();

    trace!(
        target: log::INSTR,
        stack=depth,
        method=format!("{}.{}{}", &class.name, &method.name, method.descriptor.descriptor()),
        pc,
        op=op_name(opcode)
    );

    match opcode {
        0x01 => aconst_null(thread),
        0x02 => iconst_n(thread, -1),
        0x03 => iconst_n(thread, 0),
        0x04 => iconst_n(thread, 1),
        0x05 => iconst_n(thread, 2),
        0x06 => iconst_n(thread, 3),
        0x07 => iconst_n(thread, 4),
        0x08 => iconst_n(thread, 5),
        0x09 => lconst_n(thread, 0),
        0x0A => lconst_n(thread, 1),
        0x0B => fconst_n(thread, 0.0),
        0x0C => fconst_n(thread, 1.0),
        0x0D => fconst_n(thread, 2.0),
        0x10 => bipush(thread),
        0x11 => sipush(thread),
        0x12 => load_constant(thread),
        0x13 => load_constant_wide(thread),
        0x14 => load_constant_cat_2_wide(thread),
        0x15 => iload(thread),
        0x16 => lload(thread),
        0x19 => aload(thread),
        0x1A => iload_n(thread, 0),
        0x1B => iload_n(thread, 1),
        0x1C => iload_n(thread, 2),
        0x1D => iload_n(thread, 3),
        0x22 => fload_n(thread, 0),
        0x23 => fload_n(thread, 1),
        0x24 => fload_n(thread, 2),
        0x25 => fload_n(thread, 3),
        0x2A => aload_n(thread, 0),
        0x2B => aload_n(thread, 1),
        0x2C => aload_n(thread, 2),
        0x2D => aload_n(thread, 3),
        0x32 => a_array_load(thread),
        0x34 => char_array_load(thread),
        0x36 => istore(thread),
        0x3A => astore(thread),
        0x3B => istore_n(thread, 0),
        0x3C => istore_n(thread, 1),
        0x3D => istore_n(thread, 2),
        0x3E => istore_n(thread, 3),
        0x4B => astore_n(thread, 0),
        0x4C => astore_n(thread, 1),
        0x4D => astore_n(thread, 2),
        0x4E => astore_n(thread, 3),
        0x5A => dup_x1(thread),
        0x53 => a_array_store(thread),
        0x55 => char_array_store(thread),
        0x57 => pop(thread),
        0x59 => dup(thread),
        0x60 => i_add(thread),
        0x61 => l_add(thread),
        0x64 => i_sub(thread),
        0x68 => i_mul(thread),
        0x6A => f_mul(thread),
        0x70 => irem(thread),
        0x7C => iushr(thread),
        0x79 => lshl(thread),
        0x7E => iand(thread),
        0x7F => land(thread),
        0x82 => ixor(thread),
        0x84 => i_inc(thread),
        0x85 => int_to_long(thread),
        0x86 => int_to_float(thread),
        0x8B => float_to_int(thread),
        0x95 => fcmp(thread, -1),
        0x96 => fcmp(thread, 1),
        0x99 => if_eq(thread),
        0x9A => if_ne(thread),
        0x9B => if_lt(thread),
        0x9C => if_ge(thread),
        0x9D => if_gt(thread),
        0x9E => if_le(thread),
        0xA0 => if_int_cmp_ne(thread),
        0xA1 => if_int_cmp_lt(thread),
        0xA2 => if_int_cmp_ge(thread),
        0xA4 => if_int_cmp_le(thread),
        0xA6 => if_ref_cmp_ne(thread),
        0xA7 => goto(thread),
        0xAC => i_return(thread),
        0xAE => f_return(thread),
        0xAF => d_return(thread),
        0xB0 => a_return(thread),
        0xB1 => r#return(thread),
        0xB2 => get_static(thread),
        0xB3 => put_static(thread),
        0xB4 => get_field(thread),
        0xB5 => put_field(thread),
        0xB6 => invoke_virtual(thread),
        0xB7 => invoke_special(thread),
        0xB8 => invoke_static(thread),
        0xB9 => invoke_interface(thread),
        0xBB => new(thread),
        0xBC => new_array(thread),
        0xBD => a_new_array(thread),
        0xBE => array_length(thread),
        0xBF => a_throw(thread),
        0xC0 => check_cast(thread),
        0xC1 => instance_of(thread),
        0xC2 => monitor_enter(thread),
        0xC3 => monitor_exit(thread),
        0xC6 => if_null(thread),
        0xC7 => if_non_null(thread),
        _ => panic!("not implemented opcode 0x{:0x?}", opcode)
    }
}

fn op_name(code: u8) -> &'static str {
    match code {
        0x01 => "aconst_null",
        0x02 => "iconst_m1",
        0x03 => "iconst_0",
        0x04 => "iconst_1",
        0x05 => "iconst_2",
        0x06 => "iconst_3",
        0x07 => "iconst_4",
        0x08 => "iconst_5",
        0x09 => "lconst_0",
        0x0A => "lconst_1",
        0x0B => "fconst_0",
        0x0C => "fconst_1",
        0x0D => "fconst_2",
        0x10 => "bipush",
        0x11 => "sipush",
        0x12 => "ldc",
        0x13 => "ldc_w",
        0x14 => "ldc2_w",
        0x15 => "iload",
        0x16 => "lload",
        0x19 => "aload",
        0x1A => "iload_0",
        0x1B => "iload_1",
        0x1C => "iload_2",
        0x1D => "iload_3",
        0x22 => "fload_0",
        0x23 => "fload_1",
        0x24 => "fload_2",
        0x25 => "fload_3",
        0x2A => "aload_0",
        0x2B => "aload_1",
        0x2C => "aload_2",
        0x2D => "aload_3",
        0x32 => "aaload",
        0x34 => "caload",
        0x36 => "istore",
        0x3A => "astore",
        0x3B => "istore_0",
        0x3C => "istore_1",
        0x3D => "istore_2",
        0x3E => "istore_3",
        0x4B => "astore_0",
        0x4C => "astore_1",
        0x4D => "astore_2",
        0x4E => "astore_3",
        0x5A => "dup_x1",
        0x53 => "aastore",
        0x55 => "castore",
        0x57 => "pop",
        0x59 => "dup",
        0x60 => "iadd",
        0x61 => "ladd",
        0x64 => "isub",
        0x68 => "imul",
        0x6A => "fmul",
        0x70 => "irem",
        0x7C => "iushr",
        0x79 => "lshl",
        0x7E => "iand",
        0x7F => "land",
        0x82 => "ixor",
        0x84 => "iinc",
        0x85 => "i2l",
        0x86 => "i2f",
        0x8B => "f2i",
        0x95 => "fcmpl",
        0x96 => "fcmpg",
        0x99 => "ifeq",
        0x9A => "ifne",
        0x9B => "iflt",
        0x9C => "ifge",
        0x9D => "ifgt",
        0x9E => "ifle",
        0xA0 => "if_icmpne",
        0xA1 => "if_icmplt",
        0xA2 => "if_icmpge",
        0xA4 => "if_icmple",
        0xA6 => "if_acmpne",
        0xA7 => "goto",
        0xAC => "ireturn",
        0xAE => "freturn",
        0xAF => "dreturn",
        0xB0 => "areturn",
        0xB1 => "return",
        0xB2 => "getstatic",
        0xB3 => "putstatic",
        0xB4 => "getfield",
        0xB5 => "putfield",
        0xB6 => "invokevirtual",
        0xB7 => "invokespecial",
        0xB8 => "invokestatic",
        0xB9 => "invokeinterface",
        0xBB => "new",
        0xBC => "newarray",
        0xBD => "anewarray",
        0xBE => "arraylength",
        0xBF => "athrow",
        0xC0 => "checkcast",
        0xC1 => "instanceof",
        0xC2 => "monitorenter",
        0xC3 => "monitorexit",
        0xC6 => "ifnull",
        0xC7 => "ifnonnull",
        _ => panic!("not implemented opcode 0x{:0x?}", code)
    }
}
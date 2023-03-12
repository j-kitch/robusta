use tracing::{debug, trace};
pub use new::new;

// use crate::instruction::new::{resolve_class, resolve_method};
use crate::java::{Value};
use crate::log;
use crate::method_area::const_pool::ConstPool;
use crate::method_area::Method;
use crate::thread::Thread;

pub mod new;
pub mod dup;
pub mod invoke;
pub mod field;
pub mod r#return;
pub mod r#const;
pub mod array;
pub mod math;
pub mod branch;
pub mod locals;
pub mod stack;

/// Instruction `ldc`
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.ldc).
pub fn load_constant(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u8() as u16;

    trace!(
        target: log::INSTR,
        pc=frame.pc.overflowing_sub(3).0,
        opcode="ldc",
        index
    );

    // let _guard = thread.critical_lock.acquire();
    let value = thread.runtime.method_area.resolve_category_one(frame.const_pool, index);

    frame.operand_stack.push_value(value);
}

/// astore_<n>
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.astore_n).
pub fn astore_n(thread: &mut Thread, n: u16) {
    let cur_frame = thread.stack.last_mut().unwrap();
    // let _guard = thread.critical_lock.acquire();
    let value = cur_frame.operand_stack.pop();

    trace!(
        target: log::INSTR,
        pc=cur_frame.pc-1,
        opcode=format!("astore_{}", n)
    );

    cur_frame.local_vars.store_value(n, Value::Reference(value.reference()));
}

/// istore_<n>
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.istore_n).
pub fn istore_n(thread: &mut Thread, n: u16) {
    let cur_frame = thread.stack.last_mut().unwrap();
    // let _guard = thread.critical_lock.acquire();
    let value = cur_frame.operand_stack.pop();

    trace!(
        target: log::INSTR,
        pc=cur_frame.pc-1,
        opcode=format!("istore_{}", n)
    );

    cur_frame.local_vars.store_value(n, value);
}

/// iload_<n>
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.iload_n).
pub fn iload_n(thread: &mut Thread, n: u16) {
    let cur_frame = thread.stack.last_mut().unwrap();

    trace!(
        target: log::INSTR,
        pc=cur_frame.pc-1,
        opcode=format!("iload_{}", n)
    );

    let int = cur_frame.local_vars.load_cat_one(n).int();

    // let _guard = thread.critical_lock.acquire();
    cur_frame.operand_stack.push_value(Value::Int(int));
}

/// aload_<n>
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.aload_n).
pub fn aload_n(thread: &mut Thread, n: u16) {
    let cur_frame = thread.stack.last_mut().unwrap();

    trace!(
        target: log::INSTR,
        pc=cur_frame.pc-1,
        opcode=format!("aload_{}", n)
    );

    let reference = cur_frame.local_vars.load_cat_one(n).reference();

    // let _guard = thread.critical_lock.acquire();
    cur_frame.operand_stack.push_value(Value::Reference(reference));
}

/// Instruction `return`
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.return).
pub fn r#return(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();

    trace!(
        target: log::INSTR,
        pc=frame.pc-1,
        opcode="return"
    );

    // let _guard = thread.critical_lock.acquire();
    thread.stack.pop();
}

/// Instruction `invokestatic` invokes a class static method.
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.invokestatic).
pub fn invoke_static(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let method_idx = cur_frame.read_u16();

    trace!(
        target: log::INSTR,
        pc=cur_frame.pc-3,
        opcode="invokestatic"
    );

    let method = thread.runtime.method_area.resolve_method(thread.runtime.clone(), cur_frame.const_pool, method_idx);
    let method = unsafe { method.as_ref().unwrap() };
    let class = unsafe { method.class.as_ref().unwrap() };

    if !method.is_static {
        panic!("Expected a static method");
    }

    let args = cur_frame.pop_args(true, &method.descriptor);

    if method.is_native {
        let result = thread.call_native(
            method,
           args
        );

        if let Some(result) = result {
            let cur_frame = thread.stack.last_mut().unwrap();
            cur_frame.operand_stack.push_value(result);
        }
    } else {
        debug!(target: log::INSTR, method=format!("{}.{}{}", class.name.as_str(), method.name.as_str(), method.descriptor.descriptor()), "Invoking static method");
        thread.add_frame(class.name.clone(), &class.const_pool as *const ConstPool, method as *const Method);
        let frame = thread.stack.last_mut().unwrap();
        let mut idx = 0;
        for param in &args {
            frame.local_vars.store_value(idx, param.clone());
            idx += param.category() as u16;
        }
    }
}
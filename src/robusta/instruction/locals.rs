use tracing::trace;
use crate::java::Value;
use crate::log;
use crate::thread::Thread;


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
    cur_frame.operand_stack.push(Value::Int(int));
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
    cur_frame.operand_stack.push(Value::Reference(reference));
}

pub fn istore(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u8();
    let value = frame.operand_stack.pop().int();
    frame.local_vars.store_value(index as u16, Value::Int(value));
}

pub fn astore(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u8();
    let value = frame.operand_stack.pop().reference();
    frame.local_vars.store_value(index as u16, Value::Reference(value));
}

pub fn iload(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u8();
    let value = frame.local_vars.load_cat_one(index as u16).int();
    frame.operand_stack.push(Value::Int(value));
}

pub fn aload(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u8();
    let value = frame.local_vars.load_cat_one(index as u16).reference();
    frame.operand_stack.push(Value::Reference(value));
}

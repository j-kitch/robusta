use crate::java::Value;
use crate::virtual_machine::runtime::Const;
use crate::virtual_machine::thread::Thread;

/// Instruction `ldc`
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.ldc).
pub fn load_constant(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let code = cur_frame.method.code.as_ref().unwrap();

    let index = code.code[cur_frame.pc] as u16;
    cur_frame.pc += 1;

    match cur_frame.const_pool.get_const(index) {
        Const::String(string) => {
            cur_frame.operand_stack.push(Value::Reference(string.string));
        }
        Const::Integer(int) => {
            cur_frame.operand_stack.push(Value::Int(int.int));
        }
        _ => panic!("unsupported operation")
    }
}

/// Instruction `return`
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.return).
pub fn r#return(thread: &mut Thread) {
    thread.stack.pop();
}
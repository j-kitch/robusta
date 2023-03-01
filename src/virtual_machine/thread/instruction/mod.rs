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
            panic!("string")
        }
        Const::Integer(int) => {
            panic!("err")
        }
        _ => panic!("other err")
    }
}

/// Instruction `return`
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.return).
pub fn r#return(thread: &mut Thread) {
    thread.stack.pop();
}
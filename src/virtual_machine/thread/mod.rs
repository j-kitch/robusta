use std::sync::Arc;
use crate::java::Value;

use crate::virtual_machine::runtime::{ConstPool, Method, Runtime};
use crate::virtual_machine::thread::instruction::{load_constant, r#return};

mod instruction;

/// A single Java thread in the running program.
pub struct Thread {
    /// A reference to the common runtime areas that are shared across one instance of a
    /// running program.
    runtime: Arc<Runtime>,
    /// The java virtual machine stack in this thread.
    ///
    /// The last frame on the stack is the currently active frame of the thread.
    stack: Vec<Frame>,
}

impl Thread {
    pub fn new(runtime: Arc<Runtime>, pool: Arc<ConstPool>, method: Arc<Method>) -> Self {
        Thread {
            runtime,
            stack: vec![
                Frame {
                    const_pool: pool,
                    operand_stack: OperandStack::new(),
                    method,
                    pc: 0
                }
            ]
        }
    }

    pub fn run(&mut self) {
        while !self.stack.is_empty() {
            self.next();
        }
    }

    fn next(&mut self) {
        let curr_frame = self.stack.last_mut().unwrap();
        let bytecode = &curr_frame.method.code.as_ref().unwrap().code;
        let opcode = bytecode[curr_frame.pc];
        curr_frame.pc += 1;

        match opcode {
            0x12 => load_constant(self),
            0xB1 => r#return(self),
            _ => panic!("not implemented opcode {:0x?}", opcode)
        }
    }
}

/// A single frame in a JVM thread's stack.
pub struct Frame {
    /// A reference to the related class's constant pool.
    const_pool: Arc<ConstPool>,
    /// A reference to the related method.
    method: Arc<Method>,
    operand_stack: OperandStack,
    /// The program counter within the current method.
    pc: usize,
}

/// An operand stack is used to push and pop temporary results in a frame.
///
/// For further information, see [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.6.2).
pub struct OperandStack {
    stack: Vec<Value>
}

impl OperandStack {
    pub fn new() -> Self {
        OperandStack { stack: vec![] }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
}
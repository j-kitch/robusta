use std::ops::Deref;
use std::sync::Arc;

use crate::virtual_machine::runtime::{ConstPool, Method, Runtime};

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
            0xB1 => {
                self.stack.pop();
            }
            _ => panic!("not implemented opcode {:?}", opcode)
        }
    }
}

/// A single frame in a JVM thread's stack.
pub struct Frame {
    /// A reference to the related class's constant pool.
    const_pool: Arc<ConstPool>,
    /// A reference to the related method.
    method: Arc<Method>,
    /// The program counter within the current method.
    pc: usize,
}
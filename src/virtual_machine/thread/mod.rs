use std::collections::HashMap;
use std::sync::Arc;

use crate::java::{CategoryOne, Value};
use crate::virtual_machine::runtime::{ConstPool, Method, Runtime};
use crate::virtual_machine::thread::instruction::{astore_n, istore_n, load_constant, r#return};

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
                    local_vars: LocalVars::new(),
                    method,
                    pc: 0,
                }
            ],
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
            0x3B => istore_n(self, 0),
            0x3C => istore_n(self, 1),
            0x3D => istore_n(self, 2),
            0x3E => istore_n(self, 3),
            0x4B => astore_n(self, 0),
            0x4C => astore_n(self, 1),
            0x4D => astore_n(self, 2),
            0x4E => astore_n(self, 3),
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
    local_vars: LocalVars,
    /// The program counter within the current method.
    pc: usize,
}

/// An operand stack is used to push and pop temporary results in a frame.
///
/// For further information, see [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.6.2).
pub struct OperandStack {
    stack: Vec<Value>,
}

impl OperandStack {
    pub fn new() -> Self {
        OperandStack { stack: vec![] }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
}

/// Each frame contains an array of variables called the local variables.
///
/// For further information, see [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.6.1).
pub struct LocalVars {
    map: HashMap<u16, Value>,
}

impl LocalVars {
    pub fn new() -> Self {
        LocalVars { map: HashMap::new() }
    }

    /// Store a value of category 1 (not a long or double) at the given position.
    pub fn store_cat1(&mut self, index: u16, value: CategoryOne) {
        let value = unsafe { value.int };
        self.map.insert(index, Value::Int(value));
    }
}
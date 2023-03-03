use std::collections::HashMap;
use std::sync::Arc;
use crate::instruction::{aload_n, astore_n, iload_n, invoke_static, istore_n, load_constant, new, r#return};
use crate::instruction::dup::dup;
use crate::instruction::field::get_field;
use crate::instruction::invoke::{invoke_special, invoke_virtual};
use crate::instruction::r#return::a_return;

use crate::java::{Int, Reference, Value};
use crate::runtime::{ConstPool, Method, Runtime};

/// A single Java thread in the running program.
pub struct Thread {
    /// A reference to the common runtime areas that are shared across one instance of a
    /// running program.
    pub runtime: Arc<Runtime>,
    /// The java virtual machine stack in this thread.
    ///
    /// The last frame on the stack is the currently active frame of the thread.
    pub stack: Vec<Frame>,
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

    /// Create a thread who's job is to run all the <clinit> methods of the given classes in order.
    pub fn clinit(runtime: Arc<Runtime>, classes: Vec<Arc<crate::runtime::method_area::Class>>) -> Self {
        // Reverse the classes order (want first inserted last into the stack).
        let mut classes = classes;
        classes.reverse();

        let mut thread = Thread { runtime, stack: Vec::new() };
        for class in classes.iter() {
            thread.stack.push(Frame {
                const_pool: class.const_pool.clone(),
                operand_stack: OperandStack::new(),
                local_vars: LocalVars::new(),
                method: class.methods.iter().find(|m| m.name.eq("<clinit>")).unwrap().clone(),
                pc: 0,
            });
        }

        thread
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
            0x1A => iload_n(self, 0),
            0x1B => iload_n(self, 1),
            0x1C => iload_n(self, 2),
            0x1D => iload_n(self, 3),
            0x2A => aload_n(self, 0),
            0x2B => aload_n(self, 1),
            0x2C => aload_n(self, 2),
            0x2D => aload_n(self, 3),
            0x3B => istore_n(self, 0),
            0x3C => istore_n(self, 1),
            0x3D => istore_n(self, 2),
            0x3E => istore_n(self, 3),
            0x4B => astore_n(self, 0),
            0x4C => astore_n(self, 1),
            0x4D => astore_n(self, 2),
            0x4E => astore_n(self, 3),
            0x59 => dup(self),
            0xB0 => a_return(self),
            0xB1 => r#return(self),
            0xB4 => get_field(self),
            0xB6 => invoke_virtual(self),
            0xB7 => invoke_special(self),
            0xB8 => invoke_static(self),
            0xBB => new(self),
            _ => panic!("not implemented opcode {:0x?}", opcode)
        }
    }

    /// Push a new frame onto the top of the stack.
    pub fn push_frame(&mut self, const_pool: Arc<ConstPool>, method: Arc<Method>, args: Vec<Value>) {

        let mut frame = Frame {
            const_pool,
            local_vars: LocalVars::new(),
            operand_stack: OperandStack::new(),
            pc: 0,
            method,
        };

        let mut idx = 0;
        for arg in args {
            frame.local_vars.store_value(idx, arg);
            idx += arg.width() as u16;
        }

        self.stack.push(frame);
    }
}

/// A single frame in a JVM thread's stack.
pub struct Frame {
    /// A reference to the related class's constant pool.
    pub const_pool: Arc<ConstPool>,
    /// A reference to the related method.
    pub method: Arc<Method>,
    pub operand_stack: OperandStack,
    pub local_vars: LocalVars,
    /// The program counter within the current method.
    pub pc: usize,
}

impl Frame {
    pub fn read_u16(&mut self) -> u16 {
        let bytes = &self.method.code.as_ref().unwrap().code[self.pc..self.pc+2];
        let u16 = u16::from_be_bytes(bytes.try_into().unwrap());
        self.pc += 2;
        u16
    }
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

    /// Store a value in the local vars.
    pub fn store_value(&mut self, index: u16, value: Value) {
        self.map.insert(index, value);
    }

    /// Load an int from the local vars.
    pub fn load_int(&mut self, index: u16) -> Int {
        match self.map.get(&index).unwrap() {
            Value::Int(int) => int.clone(),
            _ => panic!("expected to load int")
        }
    }

    /// Load a ref from the local vars.
    pub fn load_ref(&mut self, index: u16) -> Reference {
        match self.map.get(&index).unwrap() {
            Value::Reference(reference) => reference.clone(),
            _ => panic!("expected to load reference")
        }
    }
}
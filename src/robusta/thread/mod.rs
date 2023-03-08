use std::collections::HashMap;
use std::sync::Arc;
use tracing::debug;

use crate::instruction::{aload_n, astore_n, iload_n, invoke_static, istore_n, load_constant, new, r#return};
use crate::instruction::array::{a_new_array, array_length, char_array_load, char_array_store};
use crate::instruction::branch::{goto, if_int_cmp_ge, if_int_cmp_le};
use crate::instruction::dup::dup;
use crate::instruction::field::{get_field, put_field};
use crate::instruction::invoke::{invoke_special, invoke_virtual};
use crate::instruction::locals::{astore, iload, istore};
use crate::instruction::math::{i_add, i_inc};
use crate::instruction::new::new_array;
use crate::instruction::r#const::iconst_n;
use crate::instruction::r#return::{a_return, a_throw, i_return};
use crate::instruction::stack::{pop, sipush};
use crate::java::{CategoryOne, Int, MethodType, Value};
use crate::log;
use crate::method_area::{Class, Method};
use crate::method_area::const_pool::{ConstPool, MethodKey};
use crate::native::Args;
use crate::runtime::Runtime;

/// A single Java thread in the running program.
pub struct Thread {
    pub group: String,
    /// A reference to the common runtime areas that are shared across one instance of a
    /// running program.
    pub runtime: Arc<Runtime>,
    /// The java virtual machine stack in this thread.
    ///
    /// The last frame on the stack is the currently active frame of the thread.
    pub stack: Vec<Frame>,
}

impl Thread {
    pub fn call_native(&self, method: &Method, args: Vec<CategoryOne>) -> Option<Value> {
        self.runtime.native.call(
            method,
            &Args {
                thread: self as *const Thread,
                params: args,
                runtime: self.runtime.clone()
            }
        )
    }

    pub fn empty(runtime: Arc<Runtime>) -> Self {
        Thread { group: "thread".to_string(), runtime, stack: Vec::new() }
    }

    pub fn new(runtime: Arc<Runtime>, class: String, pool: *const ConstPool, method: *const Method) -> Self {
        Thread {
            group: "MainThread".to_string(),
            runtime,
            stack: vec![
                Frame {
                    class,
                    const_pool: pool,
                    operand_stack: OperandStack::new(),
                    local_vars: LocalVars::new(),
                    method,
                    pc: 0,
                }
            ],
        }
    }

    pub fn add_frame(&mut self, class: String, pool: *const ConstPool, method: *const Method) {
        self.stack.push(Frame {
            class,
            const_pool: pool,
            operand_stack: OperandStack::new(),
            local_vars: LocalVars::new(),
            method,
            pc: 0,
        })
    }

    /// Create a thread who's job is to run all the <clinit> methods of the given classes in order.
    pub fn clinit(runtime: Arc<Runtime>, classes: Vec<Arc<Class>>) -> Self {
        // Reverse the classes order (want first inserted last into the stack).
        let mut classes = classes;
        classes.reverse();

        let mut thread = Thread { group: "<clinit>".to_string(), runtime, stack: Vec::new() };
        for class in classes.iter() {
            thread.stack.push(Frame {
                class: class.name.clone(),
                const_pool: &class.const_pool as *const ConstPool,
                operand_stack: OperandStack::new(),
                local_vars: LocalVars::new(),
                method: class.find_method(&MethodKey {
                    class: class.name.clone(),
                    name: "<clinit>".to_string(),
                    descriptor: MethodType::from_descriptor("()V").unwrap(),
                }).unwrap(),
                pc: 0,
            });
        }

        thread
    }

    pub fn run(&mut self) {
        let class_name = self.stack.last().unwrap().class.as_str();
        let method = unsafe { self.stack.last().unwrap().method.as_ref().unwrap() };
        let method_name = format!("{}.{}{}", class_name, method.name.as_str(), method.descriptor.descriptor());
        debug!(target: log::THREAD, method=method_name, "Starting thread");
        while !self.stack.is_empty() {
            // self.runtime.heap.print_stats();
            self.next();
        }
    }

    fn next(&mut self) {
        let _ = self.stack.len();
        let curr_frame = self.stack.last_mut().unwrap();
        let method = unsafe { curr_frame.method.as_ref().unwrap() };
        let bytecode = &method.code.as_ref().unwrap().code;
        //
        // print!("{:?} {}.{}{}",
        //          std::thread::current().id() ,
        //          curr_frame.class.as_str(),
        //          curr_frame.method.name.as_str(),
        //          curr_frame.method.descriptor.descriptor());
        // io::stdout().flush().unwrap();

        let opcode = bytecode[curr_frame.pc];

        curr_frame.pc += 1;
        //
        // println!(" 0x{:00x}",
        //          opcode);

        match opcode {
            0x02 => iconst_n(self, -1),
            0x03 => iconst_n(self, 0),
            0x04 => iconst_n(self, 1),
            0x05 => iconst_n(self, 2),
            0x06 => iconst_n(self, 3),
            0x07 => iconst_n(self, 4),
            0x08 => iconst_n(self, 5),
            0x11 => sipush(self),
            0x12 => load_constant(self),
            0x15 => iload(self),
            0x1A => iload_n(self, 0),
            0x1B => iload_n(self, 1),
            0x1C => iload_n(self, 2),
            0x1D => iload_n(self, 3),
            0x2A => aload_n(self, 0),
            0x2B => aload_n(self, 1),
            0x2C => aload_n(self, 2),
            0x2D => aload_n(self, 3),
            0x34 => char_array_load(self),
            0x36 => istore(self),
            0x3A => astore(self),
            0x3B => istore_n(self, 0),
            0x3C => istore_n(self, 1),
            0x3D => istore_n(self, 2),
            0x3E => istore_n(self, 3),
            0x4B => astore_n(self, 0),
            0x4C => astore_n(self, 1),
            0x4D => astore_n(self, 2),
            0x4E => astore_n(self, 3),
            0x55 => char_array_store(self),
            0x57 => pop(self),
            0x59 => dup(self),
            0x60 => i_add(self),
            0x84 => i_inc(self),
            0xA2 => if_int_cmp_ge(self),
            0xA4 => if_int_cmp_le(self),
            0xA7 => goto(self),
            0xAC => i_return(self),
            0xB0 => a_return(self),
            0xB1 => r#return(self),
            0xB4 => get_field(self),
            0xB5 => put_field(self),
            0xB6 => invoke_virtual(self),
            0xB7 => invoke_special(self),
            0xB8 => invoke_static(self),
            0xBB => new(self),
            0xBC => new_array(self),
            0xBD => a_new_array(self),
            0xBE => array_length(self),
            0xBF => a_throw(self),
            _ => panic!("not implemented {}.{}{} opcode 0x{:0x?}", curr_frame.class.as_str(), method.name.as_str(), method.descriptor.descriptor(), opcode)
        }
    }

    /// Push a new frame onto the top of the stack.
    pub fn push_frame(&mut self, class: String, const_pool: *const ConstPool, method: *const Method, args: Vec<CategoryOne>) {
        let mut frame = Frame {
            class,
            const_pool,
            local_vars: LocalVars::new(),
            operand_stack: OperandStack::new(),
            pc: 0,
            method,
        };

        let mut idx = 0;
        for arg in args {
            frame.local_vars.store_cat_one(idx, arg);
            idx += 1;
        }

        self.stack.push(frame);
    }
}

/// A single frame in a JVM thread's stack.
pub struct Frame {
    pub class: String,
    /// A reference to the related class's constant pool.
    pub const_pool: *const ConstPool,
    /// A reference to the related method.
    pub method: *const Method,
    pub operand_stack: OperandStack,
    pub local_vars: LocalVars,
    /// The program counter within the current method.
    pub pc: usize,
}

unsafe impl Send for Frame {}

impl Frame {
    fn code(&self) -> &[u8] {
        let method = unsafe { self.method.as_ref().unwrap() };
        &method.code.as_ref().unwrap().code
    }

    pub fn read_u8(&mut self) -> u8 {
        let byte = self.code()[self.pc];
        self.pc += 1;
        byte
    }

    pub fn read_i8(&mut self) -> i8 {
        let byte = self.code()[self.pc];
        self.pc += 1;
        i8::from_be_bytes([byte])
    }

    pub fn read_u16(&mut self) -> u16 {
        let bytes = &self.code()[self.pc..self.pc + 2];
        let u16 = u16::from_be_bytes(bytes.try_into().unwrap());
        self.pc += 2;
        u16
    }

    pub fn read_i16(&mut self) -> i16 {
        let bytes = &self.code()[self.pc..self.pc + 2];
        let i16 = i16::from_be_bytes(bytes.try_into().unwrap());
        self.pc += 2;
        i16
    }
}

/// An operand stack is used to push and pop temporary results in a frame.
///
/// For further information, see [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.6.2).
pub struct OperandStack {
    stack: Vec<u8>
}

impl OperandStack {
    pub fn new() -> Self {
        OperandStack { stack: vec![] }
    }

    pub fn push_cat_one(&mut self, cat_one: CategoryOne) {
        let i32 = unsafe { cat_one.int.0 };
        for byte in i32.to_be_bytes() {
            self.stack.push(byte);
        }
    }

    pub fn push_value(&mut self, value: Value) {
        if value.category() == 1 {
            return self.push_cat_one(value.cat_one());
        }
        panic!("Not implemented")
    }

    pub fn pop_cat_one(&mut self) -> CategoryOne {
        let new_len = self.stack.len() - 4;
        let bytes = self.stack.drain(new_len..);
        CategoryOne { int: Int(i32::from_be_bytes(bytes.as_slice().try_into().unwrap())) }
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

    pub fn store_cat_one(&mut self, index: u16, value: CategoryOne) {
        self.store_value(index, Value::Int(value.int()));
    }

    /// Load an int from the local vars.
    pub fn load_cat_one(&mut self, index: u16) -> CategoryOne {
        match self.map.get(&index).unwrap() {
            Value::Int(int) => CategoryOne { int: *int },
            _ => panic!("expected to load int")
        }
    }
}
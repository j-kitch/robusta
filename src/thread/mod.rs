use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::class::{Class, Method};
use crate::heap::{Ref, Value};
use crate::runtime::Runtime;
use crate::thread::local_vars::{Locals, LocalVars};
use crate::thread::op_stack::{OperandStack, OpStack};

mod op;
pub mod local_vars;
pub mod op_stack;

pub struct Thread {
    pub rt: Rc<RefCell<Runtime>>,
    pub frames: Vec<Frame>,
}

pub struct Frame {
    pub pc: u32,
    pub class: Rc<Class>,
    pub method: Rc<Method>,
    pub local_vars: LocalVars,
    pub op_stack: OperandStack,
}

impl Thread {
    pub fn new(rt: Runtime) -> Self {
        Thread { rt: Rc::new(RefCell::new(rt)), frames: vec![] }
    }

    pub fn create_frame(&mut self, class: Rc<Class>, method: Rc<Method>, args: Vec<Value>) {
        let mut frame = Frame {
            pc: 0,
            class: class.clone(),
            local_vars: LocalVars::new(method.max_locals.clone()),
            op_stack: OperandStack::new(method.max_stack.clone()),
            method,
        };
        let mut idx = 0;
        for arg in args.iter() {
            match arg {
                Value::Ref(val) => {
                    frame.local_vars.store_ref(idx, val.clone());
                    idx += 1;
                }
            }
        }
        self.frames.push(frame);
    }

    pub fn run(&mut self) {
        while self.alive() {
            self.next();
        }
    }

    fn alive(&self) -> bool {
        !self.frames.is_empty()
    }

    fn frame_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    fn frame(&self) -> &Frame {
        self.frames.last().unwrap()
    }

    fn read_u8(&mut self) -> u8 {
        self.frame_mut().read_u8()
    }

    fn read_i8(&mut self) -> i8 {
        let u8 = self.frame_mut().read_u8();
        i8::from_be_bytes([u8])
    }

    fn read_i16(&mut self) -> i16 {
        self.frame_mut().read_i16()
    }

    fn read_u16(&mut self) -> u16 {
        self.frame_mut().read_u16()
    }

    fn next(&mut self) {
        let curr = self.frame_mut();
        let op_code = curr.read_u8();
        let op = op::get_op(curr, op_code);
        op(self);
    }

    pub fn load(&mut self, class_name: &str) -> Option<Rc<Class>> {
        let rt = self.rt.clone();
        let rt = rt.deref();
        let mut rt = rt.borrow_mut();
        let rt = rt.deref_mut();
        let class_loader = rt.class_loader.borrow_mut();
        class_loader.load(class_name)
    }

    pub fn object(&self, key: u32) -> Rc<RefCell<Ref>> {
        self.rt.as_ref().borrow().deref().heap.get(key)
    }
}

impl OpStack for Thread {
    fn push_ref(&mut self, value: u32) {
        self.frame_mut().op_stack.push_ref(value)
    }

    fn push_int(&mut self, value: i32) {
        self.frame_mut().op_stack.push_int(value)
    }

    fn pop_ref(&mut self) -> u32 {
        self.frame_mut().op_stack.pop_ref()
    }

    fn pop_int(&mut self) -> i32 {
        self.frame_mut().op_stack.pop_int()
    }
}

impl Locals for Thread {
    fn store_ref(&mut self, idx: u16, value: u32) {
        self.frame_mut().local_vars.store_ref(idx, value)
    }

    fn store_int(&mut self, idx: u16, value: i32) {
        self.frame_mut().local_vars.store_int(idx, value)
    }

    fn load_ref(&self, idx: u16) -> u32 {
        self.frame().local_vars.load_ref(idx)
    }

    fn load_int(&self, idx: u16) -> i32 {
        self.frame().local_vars.load_int(idx)
    }
}

impl Frame {
    fn read_u8(&mut self) -> u8 {
        let u8 = self.method.code.get(self.pc as usize).unwrap().clone();
        self.pc += 1;
        u8
    }

    fn read_i16(&mut self) -> i16 {
        let mut bytes = [0, 0];
        bytes[0] = self.read_u8();
        bytes[1] = self.read_u8();
        i16::from_be_bytes(bytes)
    }

    fn read_u16(&mut self) -> u16 {
        let mut bytes = [0, 0];
        bytes[0] = self.read_u8();
        bytes[1] = self.read_u8();
        u16::from_be_bytes(bytes)
    }
}

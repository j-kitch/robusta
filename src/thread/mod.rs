use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::class::{Class, Method};
use crate::heap::Ref;
use crate::runtime::Runtime;
use crate::thread::local_vars::LocalVars;
use crate::thread::op_stack::OperandStack;

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
    pub fn run(&mut self) {
        while self.alive() {
            self.next();
        }
    }

    fn alive(&self) -> bool {
        !self.frames.is_empty()
    }

    fn curr(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    fn read_u8(&mut self) -> u8 {
        self.curr().read_u8()
    }

    fn read_i8(&mut self) -> i8 {
        let u8 = self.curr().read_u8();
        i8::from_be_bytes([u8])
    }

    fn read_i16(&mut self) -> i16 {
        self.curr().read_i16()
    }

    fn read_u16(&mut self) -> u16 {
        self.curr().read_u16()
    }

    fn next(&mut self) {
        let curr = self.curr();
        let op_code = curr.read_u8();
        let op = op::get_op(curr, op_code);
        op(self);
    }

    pub fn load(&mut self, class_name: &str) -> Option<Rc<Class>> {
        let mut rt = self.rt.clone();
        let mut rt = rt.deref();
        let mut rt = rt.borrow_mut();
        let mut rt = rt.deref_mut();
        let mut class_loader = rt.class_loader.borrow_mut();
        class_loader.load(class_name)
    }

    pub fn object(&self, key: u32) -> Rc<RefCell<Ref>> {
        self.rt.as_ref().borrow().deref().heap.get(key)
    }

    pub fn pop_ref(&mut self) -> u32 {
        self.curr().op_stack.pop_ref()
    }

    pub fn push_ref(&mut self, op: u32) {
        self.curr().op_stack.push_ref(op);
    }

    pub fn pop_int(&mut self) -> i32 {
        self.curr().op_stack.pop_int()
    }

    pub fn push_int(&mut self, op: i32) {
        self.curr().op_stack.push_int(op);
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

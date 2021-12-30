use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::class::{Class, Method};
use crate::heap::{Ref, Value};
use crate::runtime::Runtime;
use crate::thread::local_vars::LocalVars;
use crate::thread::op_stack::OperandStack;

mod op;
pub mod local_vars;
pub mod op_stack;

pub struct Thread {
    pub rt: Rc<RefCell<Runtime>>,
    pub frames: FrameStack,
}

pub struct FrameStack {
    current: Option<Frame>,
    rest: Vec<Frame>,
}

impl FrameStack {
    pub fn has_frames(&self) -> bool {
        self.current.is_some()
    }

    pub fn current_mut(&mut self) -> &mut Frame {
        self.current.as_mut().unwrap()
    }

    pub fn current(&self) -> &Frame {
        self.current.as_ref().unwrap()
    }

    pub fn push(&mut self, frame: Frame) -> &mut Frame {
        if let Some(old_current) = self.current.take() {
            self.rest.push(old_current);
        }
        self.current = Some(frame);
        self.current_mut()
    }

    pub fn pop(&mut self) -> Option<&mut Frame> {
        self.current = self.rest.pop();
        self.current.as_mut()
    }
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
        Thread {
            rt: Rc::new(RefCell::new(rt)),
            frames: FrameStack {
                current: None,
                rest: vec![],
            },
        }
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
        while self.frames.has_frames() {
            self.next();
        }
    }

    fn next(&mut self) {
        let current = self.frames.current_mut();
        let op_code = current.read_u8();
        let op = op::get_op(current, op_code);
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

impl Frame {
    fn read_i8(&mut self) -> i8 {
        let bytes = [self.read_u8(); 1];
        i8::from_be_bytes(bytes)
    }

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

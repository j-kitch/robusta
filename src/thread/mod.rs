use std::rc::Rc;

use crate::class::{Class, Method};

pub struct Thread<'a> {
    pub frames: Vec<Frame<'a>>,
}

pub struct Frame<'a> {
    pub pc: u32,
    pub class: Rc<Class>,
    pub method: &'a Method,
}

impl<'a> Thread<'a> {
    pub fn run(&mut self) {
        while !self.frames.is_empty() {
            self.next();
        }
    }

    fn next(&mut self) {
        let frame = self.frames.last_mut().unwrap();
        let op = frame.method.code.get(frame.pc as usize).unwrap().clone();
        panic!("Unknown op at {}.{}{} PC {} {:#02x}",
               &frame.class.this_class,
               &frame.method.name,
               &frame.method.descriptor,
               frame.pc,
               op);
    }
}

mod op;

use std::rc::Rc;

use crate::class::{Class, Method};

pub struct Thread {
    pub frames: Vec<Frame>,
}

pub struct Frame {
    pub pc: u32,
    pub class: Rc<Class>,
    pub method: Rc<Method>,
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

    fn next(&mut self) {
        let curr = self.curr();
        let op_code = curr.read_u8();
        let op = op::get_op(curr, op_code);
        op(self);
    }
}

impl Frame {
    fn read_u8(&mut self) -> u8 {
        let u8 = self.method.code.get(self.pc as usize).unwrap().clone();
        self.pc += 1;
        u8
    }
}

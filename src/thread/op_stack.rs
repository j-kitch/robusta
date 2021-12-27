pub struct OperandStack {
    stack: Vec<u32>
}

impl OperandStack {
    pub fn new(max_stack: u16) -> Self {
        OperandStack { stack: Vec::with_capacity(max_stack as usize) }
    }

    pub fn push_ref(&mut self, op: u32) {
        self.stack.push(op);
    }

    pub fn pop_ref(&mut self) -> u32 {
        self.stack.pop().unwrap()
    }
}

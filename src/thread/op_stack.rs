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

    pub fn push_int(&mut self, op: i32) {
        let bytes: [u8; 4] = op.to_be_bytes();
        let u32 = u32::from_be_bytes(bytes);
        self.stack.push(u32);
    }

    pub fn pop_ref(&mut self) -> u32 {
        self.stack.pop().unwrap()
    }

    pub fn pop_int(&mut self) -> i32 {
        let u32 = self.pop_ref();
        let bytes: [u8; 4] = u32.to_be_bytes();
        i32::from_be_bytes(bytes)
    }
}

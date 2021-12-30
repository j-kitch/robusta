pub struct OperandStack {
    stack: Vec<u8>,
}

impl OperandStack {
    pub fn new(max_stack: u16) -> Self {
        OperandStack { stack: Vec::with_capacity((max_stack as usize) * 4) }
    }

    fn push_bytes(&mut self, word: &[u8]) {
        self.stack.extend(word);
    }

    fn pop_word(&mut self) -> [u8; 4] {
        let mut word = [0; 4];
        for i in (0..4).rev() {
            word[i] = self.stack.pop().unwrap();
        }
        word
    }

    pub fn push_ref(&mut self, op: u32) {
        self.push_bytes(&op.to_be_bytes())
    }

    pub fn push_int(&mut self, op: i32) {
        self.push_bytes(&op.to_be_bytes())
    }

    pub fn pop_ref(&mut self) -> u32 {
        u32::from_be_bytes(self.pop_word())
    }

    pub fn pop_int(&mut self) -> i32 {
        i32::from_be_bytes(self.pop_word())
    }
}

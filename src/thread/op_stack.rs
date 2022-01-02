use crate::heap::Value;

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

    fn pop_dword(&mut self) -> [u8; 8] {
        let mut dword = [0; 8];
        for i in (0..8).rev() {
            dword[i] = self.stack.pop().unwrap();
        }
        dword
    }

    pub fn push_ref(&mut self, op: u32) {
        self.push_bytes(&op.to_be_bytes())
    }

    pub fn push_int(&mut self, op: i32) {
        self.push_bytes(&op.to_be_bytes())
    }

    pub fn push_long(&mut self, op: i64) {
        self.push_bytes(&op.to_be_bytes())
    }

    pub fn push_float(&mut self, op: f32) {
        self.push_bytes(&op.to_be_bytes())
    }

    pub fn push_double(&mut self, op: f64) {
        self.push_bytes(&op.to_be_bytes())
    }

    pub fn push(&mut self, op: Value) {
        match op {
            Value::Ref(op) => self.push_ref(op),
            Value::Int(op) => self.push_int(op),
            Value::Long(op) => self.push_long(op),
            Value::Float(op) => self.push_float(op),
            Value::Double(op) => self.push_double(op),
        }
    }

    pub fn pop_ref(&mut self) -> u32 {
        u32::from_be_bytes(self.pop_word())
    }

    pub fn pop_int(&mut self) -> i32 {
        i32::from_be_bytes(self.pop_word())
    }

    pub fn pop_float(&mut self) -> f32 {
        f32::from_be_bytes(self.pop_word())
    }

    pub fn pop_double(&mut self) -> f64 {
        f64::from_be_bytes(self.pop_dword())
    }

    pub fn pop_long(&mut self) -> i64 {
        i64::from_be_bytes(self.pop_dword())
    }
}

use crate::heap::Value;

pub struct LocalVars {
    bytes: Vec<u8>,
}

impl LocalVars {
    pub fn new(max_vars: u16) -> Self {
        LocalVars { bytes: vec![0; max_vars as usize * 4] }
    }

    fn store_bytes(&mut self, idx: u16, bytes: &[u8]) {
        let mut idx = idx as usize * 4;
        for byte in bytes {
            self.bytes[idx] = byte.clone();
            idx += 1;
        }
    }

    fn load_word(&self, idx: u16) -> [u8; 4] {
        let idx = idx as usize * 4;
        let mut bytes = [0; 4];
        for i in 0..4 {
            bytes[i]  = self.bytes[idx + i];
        }
        bytes
    }

    fn load_dword(&self, idx: u16) -> [u8; 8] {
        let idx = idx as usize * 4;
        let mut bytes = [0; 8];
        for i in 0..8 {
            bytes[i] = self.bytes[idx + i];
        }
        bytes
    }

    pub fn store_ref(&mut self, idx: u16, value: u32) {
        let bytes = value.to_be_bytes();
        self.store_bytes(idx, &bytes)
    }

    pub fn store_int(&mut self, idx: u16, value: i32) {
        let bytes = value.to_be_bytes();
        self.store_bytes(idx, &bytes)
    }

    pub fn store_long(&mut self, idx: u16, value: i64) {
        let bytes = value.to_be_bytes();
        self.store_bytes(idx, &bytes);
    }

    pub fn store_float(&mut self, idx: u16, value: f32) {
        let bytes = value.to_be_bytes();
        self.store_bytes(idx, &bytes);
    }

    pub fn store_double(&mut self, idx: u16, value: f64) {
        let bytes = value.to_be_bytes();
        self.store_bytes(idx, &bytes);
    }

    pub fn store_value(&mut self, idx: u16, value: Value) {
        match value {
            Value::Ref(r) => self.store_ref(idx, r),
            Value::Int(i) => self.store_int(idx, i),
            Value::Long(l) => self.store_long(idx, l),
            Value::Float(f) => self.store_float(idx, f),
            Value::Double(d) => self.store_double(idx, d),
        }
    }

    pub fn load_ref(&self, idx: u16) -> u32 {
        let word = self.load_word(idx);
        u32::from_be_bytes(word)
    }

    pub fn load_int(&self, idx: u16) -> i32 {
        let word = self.load_word(idx);
        i32::from_be_bytes(word)
    }

    pub fn load_long(&self, idx: u16) -> i64 {
        let dword = self.load_dword(idx);
        i64::from_be_bytes(dword)
    }

    pub fn load_float(&self, idx: u16) -> f32 {
        let word = self.load_word(idx);
        f32::from_be_bytes(word)
    }

    pub fn load_double(&self, idx: u16) -> f64 {
        let dword = self.load_dword(idx);
        f64::from_be_bytes(dword)
    }
}

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

    pub fn store_ref(&mut self, idx: u16, value: u32) {
        let bytes = value.to_be_bytes();
        self.store_bytes(idx, &bytes)
    }

    pub fn store_int(&mut self, idx: u16, value: i32) {
        let bytes = value.to_be_bytes();
        self.store_bytes(idx, &bytes)
    }

    pub fn load_ref(&self, idx: u16) -> u32 {
        let word = self.load_word(idx);
        u32::from_be_bytes(word)
    }

    pub fn load_int(&self, idx: u16) -> i32 {
        let word = self.load_word(idx);
        i32::from_be_bytes(word)
    }
}

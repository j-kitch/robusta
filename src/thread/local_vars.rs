pub struct LocalVars {
    bytes: Vec<u8>,
}

impl LocalVars {
    pub fn new(max_vars: u16) -> Self {
        LocalVars { bytes: vec![0; max_vars as usize * 4] }
    }

    pub fn store_ref(&mut self, idx: u16, value: u32) {
        let bytes: [u8; 4] = value.to_be_bytes();
        for i in 0..4 {
            let idx = (idx * 4) + i;
            let idx = idx as usize;

            self.bytes[idx] = bytes[i as usize];
        }
    }

    pub fn store_int(&mut self, idx: u16, value: i32) {
        let bytes: [u8; 4] = value.to_be_bytes();
        for i in 0..4 {
            let idx = (idx * 4) + i;
            let idx = idx as usize;

            self.bytes[idx] = bytes[i as usize];
        }
    }

    pub fn load_ref(&self, idx: u16) -> u32 {
        let idx = (idx * 4) as usize;
        let mut bytes = [0; 4];
        for i in 0..4 {
            bytes[i] = self.bytes[idx + i];
        }
        u32::from_be_bytes(bytes)
    }

    pub fn load_int(&self, idx: u16) -> i32 {
        let idx = (idx * 4) as usize;
        let mut bytes = [0; 4];
        for i in 0..4 {
            bytes[i] = self.bytes[idx + i];
        }
        i32::from_be_bytes(bytes)
    }
}

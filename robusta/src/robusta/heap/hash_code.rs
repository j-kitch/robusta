use std::sync::Mutex;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use crate::java::Int;

const RNG_SEED: u64 = 542364236435;

pub struct HashCode {
    rng: Mutex<StdRng>
}

impl HashCode {
    pub fn new() -> Self {
        HashCode {
            rng: Mutex::new(StdRng::seed_from_u64(RNG_SEED))
        }
    }

    pub fn next(&self) -> Int {
        let mut rng = self.rng.lock().unwrap();
        Int(rng.gen())
    }
}
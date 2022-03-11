use std::collections::HashMap;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::descriptor::MethodDescriptor;
use crate::heap::Value;
use crate::heap::Value::Int;
use crate::native::NativePlugin;
use crate::runtime::Runtime;

static SEED: u64 = 10;

pub struct HashCodePlugin {
    codes: HashMap<u32, i32>,
    rng: StdRng,
}

impl HashCodePlugin {
    pub fn new() -> Self {
        HashCodePlugin { codes: HashMap::new(), rng: StdRng::seed_from_u64(SEED) }
    }
}

impl NativePlugin for HashCodePlugin {


    fn supports(&self, class: &str, name: &str, desc: &MethodDescriptor) -> bool {
        class.eq("java/lang/Object") && name.eq("hashCode")
            && desc.eq(&MethodDescriptor::parse("()I"))
    }

    fn invoke(&mut self, _: &mut Runtime, args: Vec<Value>) -> Option<Value> {
        let this_ref = args[0].reference();

        if !self.codes.contains_key(&this_ref) {
            let next_hash_code = self.rng.gen();
            self.codes.insert(this_ref, next_hash_code);
        }

        let hash_code = self.codes.get(&this_ref).unwrap().clone();
        Some(Int(hash_code))
    }
}

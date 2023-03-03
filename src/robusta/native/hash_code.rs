use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use rand::{Rng, thread_rng};

use crate::java::{Int, MethodType, Reference, Value};
use crate::native::plugin::{Args, Method, Plugin};

pub fn hash_code_plugins() -> Vec<Box<dyn Plugin>> {
    let library = Arc::new(HashCodeLibrary { hash_codes: RwLock::new(HashMap::new()) });

    let get_hash_code: Box<dyn Plugin> = Box::new(GetHashCode { library: library.clone() }) as _;
    let register_hash_code: Box<dyn Plugin> = Box::new(RegisterHashCode { library: library.clone() }) as _;

    vec![get_hash_code, register_hash_code]
}

struct HashCodeLibrary {
    hash_codes: RwLock<HashMap<Reference, Int>>,
}

struct GetHashCode {
    library: Arc<HashCodeLibrary>,
}

impl Plugin for GetHashCode {
    fn supports(&self, method: &Method) -> bool {
        method.class.eq("java.lang.Object") &&
            method.name.eq("hashCode") &&
            method.descriptor.eq(&MethodType::from_descriptor("()I").unwrap())
    }

    fn call(&self, _: &Method, args: &Args) -> Option<Value> {
        let reference = args.params[0].reference();

        let codes = self.library.hash_codes.read().unwrap();

        let int = codes.get(&reference).unwrap();

        Some(Value::Int(int.clone()))
    }
}

struct RegisterHashCode {
    library: Arc<HashCodeLibrary>,
}

impl Plugin for RegisterHashCode {
    fn supports(&self, method: &Method) -> bool {
        method.class.eq("java.lang.Object") &&
            method.name.eq("registerHashCode") &&
            method.descriptor.eq(&MethodType::from_descriptor("()V").unwrap())
    }

    fn call(&self, method: &Method, args: &Args) -> Option<Value> {
        let reference = args.params[0].reference();
        let int: Int = Int(thread_rng().gen());

        let mut codes = self.library.hash_codes.write().unwrap();
        codes.insert(reference, int);

        None
    }
}
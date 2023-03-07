use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::RwLock;
use rand::prelude::ThreadRng;
use rand::{RngCore, thread_rng};

use crate::collection::once::OnceMap;
use crate::heap::allocator::{Allocator, Array, ArrayType, Object};
use crate::java::{Int, Reference};
use crate::method_area::Class;

mod allocator;

pub struct Heap {
    allocator: Allocator,
    references: RwLock<HashMap<Reference, Heaped>>,
    class_objects: OnceMap<String, Reference>,
    string_constants: OnceMap<String, Reference>,
}

impl Heap {
    pub fn new_object(&self, class: &Class) -> Reference {
        let object = self.allocator.new_object(class);
        self.insert(Heaped::Object(object))
    }

    pub fn new_array(&self, arr_type: ArrayType, length: Int) -> Reference {
        let array = self.allocator.new_array(arr_type, length);
        self.insert(Heaped::Array(array))
    }

    fn insert(&self, heaped: Heaped) -> Reference {
        let mut rng = thread_rng();
        let mut references = self.references.write().unwrap();

        // TODO: This is probably an awful way to allocate references!
        let reference = {
            let mut next_ref = Reference(rng.next_u32());
            while references.contains_key(&next_ref) {
                next_ref = Reference(rng.next_u32());
            }
            next_ref
        };

        references.insert(reference, heaped);
        reference
    }
}

enum Heaped {
    Object(Object),
    Array(Array),
}
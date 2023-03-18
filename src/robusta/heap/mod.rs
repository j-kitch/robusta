use std::collections::{HashMap, HashSet};
use std::sync::RwLock;

use rand::{RngCore, thread_rng};

use crate::collection::once::OnceMap;
use crate::heap::allocator::{Allocator, Array, ArrayType, Object};
use crate::java::{CategoryOne, FieldType, Int, Reference};
use crate::method_area::Class;
use crate::method_area::const_pool::FieldKey;

pub mod allocator;
mod hash_code;
pub mod garbage_collector;
pub mod sync;

pub struct Heap {
    pub allocator: Allocator,
    references: RwLock<HashMap<Reference, Heaped>>,
    class_objects: OnceMap<String, Reference>,
    string_constants: OnceMap<String, Reference>,
    static_objects: OnceMap<String, Reference>,
}

unsafe impl Send for Heap {}

impl Heap {
    pub fn retain(&self, retain: &HashSet<Reference>) {
        let mut references = self.references.write().unwrap();

        let refs_to_remove: Vec<Reference> = references.keys()
            .filter(|key| !retain.contains(key))
            .map(|key| *key)
            .collect();

        for reference in &refs_to_remove {
            references.remove(reference);
        }
    }

    pub fn new() -> Self {
        Heap {
            allocator: Allocator::new(),
            references: RwLock::new(HashMap::new()),
            class_objects: OnceMap::new(),
            string_constants: OnceMap::new(),
            static_objects: OnceMap::new(),
            // safe_point: AtomicBool::new(false),
        }
    }

    pub fn new_object(&self, class: &Class) -> Reference {
        let object = self.allocator.new_object(class);
        self.insert(Heaped::Object(object))
    }

    pub fn get(&self, reference: Reference) -> Heaped {
        let mut references = self.references.write().unwrap();
        let heaped = references.get_mut(&reference);
        heaped.unwrap().clone()
    }

    pub fn set(&self, reference: Reference, heaped: Heaped) {
        let mut references = self.references.write().unwrap();
        references.insert(reference, heaped);
    }

    pub fn get_static(&self, class: &Class) -> Reference {
        self.static_objects.get_or_init(class.name.clone(), |_| {
            let object = self.allocator.new_static_object(class);
            self.insert(Heaped::Object(object))
        }).clone()
    }

    pub fn new_array(&self, arr_type: ArrayType, length: Int) -> Reference {
        let array = self.allocator.new_array(arr_type, length);
        self.insert(Heaped::Array(array))
    }

    pub fn get_object(&self, reference: Reference) -> Object {
        let references = self.references.read().unwrap();
        match references.get(&reference).unwrap() {
            Heaped::Object(object) => object.clone(),
            _ => panic!("")
        }
    }

    pub fn get_string(&self, reference: Reference) -> String {
        let string_obj = self.get_object(reference);
        let chars_ref = string_obj.get_field(&FieldKey {
            class: "java.lang.String".to_string(),
            name: "value".to_string(),
            descriptor: FieldType::from_descriptor("[C").unwrap(),
        }).reference();
        let chars_arr = self.get_array(chars_ref);
        let chars_arr = chars_arr.as_chars_slice();
        String::from_utf16(chars_arr).unwrap()
    }

    pub fn get_array(&self, reference: Reference) -> Array {
        let references = self.references.read().unwrap();
        match references.get(&reference).unwrap() {
            Heaped::Array(array) => array.clone(),
            _ => panic!("")
        }
    }

    pub fn insert_string_const(&self, string: &str, class: &Class) -> Reference {
        self.string_constants.get_or_init(string.to_string(), |string| {
            // Chars
            let chars: Vec<u16> = string.encode_utf16().collect();
            let chars_ref = self.new_array(ArrayType::Char, Int(chars.len() as i32));
            let char_array = self.get_array(chars_ref);
            let char_array = char_array.as_chars_mut();
            char_array.copy_from_slice(&chars);

            // String
            let object_ref = self.new_object(class);
            let object = self.get_object(object_ref);

            object.set_field(&FieldKey {
                class: "java.lang.String".to_string(),
                name: "value".to_string(),
                descriptor: FieldType::from_descriptor("[C").unwrap(),
            }, CategoryOne { reference: chars_ref });

            object_ref
        }).clone()
    }

    pub fn insert_class_object(&self, class: &Class, class_class: &Class, string_class: &Class) -> Reference {
        self.class_objects.get_or_init(class.name.clone(), |_| {
            // Name
            let name_ref = self.insert_string_const(&class.name, string_class);

            // Class
            let object_ref = self.new_object(class_class);
            let object = self.get_object(object_ref);

            object.set_field(&FieldKey {
                class: "java.lang.Class".to_string(),
                name: "name".to_string(),
                descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
            }, CategoryOne { reference: name_ref });

            object_ref
        }).clone()
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

    pub fn get_thread_alive(&self, thread: Reference) -> bool {
        let thread_obj = self.get_object(thread);

        let value = thread_obj.get_field(&FieldKey {
            class: "java.lang.Thread".to_string(),
            name: "threadStatus".to_string(),
            descriptor: FieldType::Int,
        }).int().0;

        value == 1
    }

    pub fn start_thread(&self, thread: Reference) {
        let thread_obj = self.get_object(thread);
        thread_obj.set_field(&FieldKey {
            class: "java.lang.Thread".to_string(),
            name: "threadStatus".to_string(),
            descriptor: FieldType::Int,
        }, CategoryOne { int: Int(1) });
    }

    pub fn end_thread(&self, thread: Reference) {
        let thread_obj = self.get_object(thread);
        thread_obj.set_field(&FieldKey {
            class: "java.lang.Thread".to_string(),
            name: "threadStatus".to_string(),
            descriptor: FieldType::Int,
        }, CategoryOne { int: Int(2) });
    }
}

#[derive(Clone, Copy)]
pub enum Heaped {
    Object(Object),
    Array(Array),
}
use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

use rand::{RngCore, thread_rng};

use crate::java::{FieldType, Int, Reference};
use crate::runtime::const_pool;
use crate::runtime::const_pool::Field;
pub use crate::runtime::heap3::{Array, ArrayType, HeapInner, Object};
use crate::runtime::method_area::Class;

pub struct Heap {
    inner: HeapInner,
    values: RwLock<HashMap<Reference, HeapValue>>,
    string_consts: RwLock<HashMap<String, Reference>>,
    class_objects: RwLock<HashMap<String, Reference>>,
}

impl Heap {
    pub fn print_stats(&self) {
        self.inner.print_stats();
        let values = self.values.read().unwrap();
        println!("{} objects in the heap", values.len());
    }

    fn insert(values: &mut RwLockWriteGuard<HashMap<Reference, HeapValue>>, value: HeapValue) -> Reference {
        let mut rng = thread_rng();
        let mut reference = rng.next_u32();
        loop {
            if !values.contains_key(&Reference(reference)) {
                values.insert(Reference(reference), value);
                break;
            }
            reference = rng.next_u32();
        }
        Reference(reference)
    }

    pub fn new() -> Arc<Self> {
        Arc::new(Heap {
            inner: HeapInner::new(),
            values: RwLock::new(HashMap::new()),
            string_consts: RwLock::new(HashMap::new()),
            class_objects: RwLock::new(HashMap::new()),
        })
    }

    pub fn load_object(self: &Arc<Self>, reference: Reference) -> Arc<Object> {
        let values = self.values.read().unwrap();
        match values.get(&reference).unwrap() {
            HeapValue::Object(object) => object.clone(),
            _ => panic!("expected object")
        }
    }

    pub fn load_array(self: &Arc<Self>, reference: Reference) -> Arc<Array> {
        let values = self.values.read().unwrap();
        match values.get(&reference).unwrap() {
            HeapValue::Array(array) => array.clone(),
            _ => panic!("expected array")
        }
    }

    pub fn insert_char_array(self: &Arc<Self>, length: usize) -> Reference {
        let char_arr = self.inner.allocator.new_array(ArrayType::Char, Int(length as i32));
        let mut values = self.values.write().unwrap();

        let arr_ref = Heap::insert(&mut values, HeapValue::Array(Arc::new(char_arr)));

        arr_ref
    }

    pub fn insert_char_arr(self: &Arc<Self>, chars: &[u16]) -> Reference {
        let char_arr = self.inner.allocator.new_array(ArrayType::Char, Int(chars.len() as i32));
        let char_slice = char_arr.as_chars_mut();
        char_slice.copy_from_slice(chars);
        Heap::insert(&mut self.values.write().unwrap(), HeapValue::Array(Arc::new(char_arr)))
    }

    pub fn insert_new(self: &Arc<Self>, class: &Arc<Class>) -> Reference {
        let class = self.inner.add_class(class.clone());
        let object = self.inner.allocator.new_object(class.clone());

        // TODO: Extremely poor code here - very very temporary!
        let mut values = self.values.write().unwrap();

        Heap::insert(&mut values, HeapValue::Object(Arc::new(object)))
    }

    pub fn get_class(self: &Arc<Self>, class_name: &str) -> Reference {
        let class_objects = self.class_objects.read().unwrap();
        class_objects.get(class_name).unwrap().clone()
    }

    pub fn get_string(self: &Arc<Self>, string: &str) -> Reference {
        let string_consts = self.string_consts.read().unwrap();
        string_consts.get(string).unwrap().clone()
    }

    pub fn intern_string(self: &Arc<Self>, string_ref: Reference) -> Reference {
        let values = self.values.read().unwrap();
        let string_obj = values.get(&string_ref).unwrap();
        let string_obj = if let HeapValue::Object(obj) = string_obj {
            obj
        } else {
            panic!()
        };

        let chars_ref = string_obj.get_field(&Field {
            name: "chars".to_string(),
            descriptor: FieldType::Array(Box::new(FieldType::Char)),
            class: Arc::new(const_pool::Class { name: "java.lang.String".to_string() }),
        }).reference();

        let chars_arr = values.get(&chars_ref).unwrap();
        let chars_arr = if let HeapValue::Array(arr) = chars_arr {
            arr
        } else {
            panic!()
        };

        let chars = chars_arr.as_chars_slice();
        let string = String::from_utf16(chars).unwrap();

        let mut interned_strings = self.string_consts.write().unwrap();
        if interned_strings.contains_key(string.as_str()) {
            interned_strings.get(string.as_str()).unwrap().clone()
        } else {
            interned_strings.insert(string, string_ref);
            string_ref
        }
    }

    pub fn intern_class(&self, name: &str, object_ref: Reference) -> Reference {
        let mut class_objects = self.class_objects.write().unwrap();
        if let Some(old_ref) = class_objects.get(name) {
            *old_ref
        } else {
            class_objects.insert(name.to_string(), object_ref);
            object_ref
        }
    }
}

pub enum HeapValue {
    Array(Arc<Array>),
    Object(Arc<Object>),
}

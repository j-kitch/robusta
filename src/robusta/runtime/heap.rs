use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

use rand::{RngCore, thread_rng};

use crate::java::{FieldType, Int, Reference, Value};
pub use crate::runtime::heap3::{Array, ArrayType, HeapInner, Object};
use crate::runtime::{const_pool, Runtime};
use crate::runtime::const_pool::Field;
use crate::runtime::method_area::Class;

pub struct Heap {
    inner: HeapInner,
    values: RwLock<HashMap<Reference, HeapValue>>,
    string_consts: RwLock<HashMap<String, Reference>>,
    class_objects: RwLock<HashMap<String, Reference>>
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

    pub fn get_class_object(self: &Arc<Self>, runtime: Arc<Runtime>, class: &str, class_class: &Arc<Class>) -> Reference {
        let mut values = self.values.write().unwrap();
        let mut classes = self.class_objects.write().unwrap();

        if let Some(reference) = classes.get(class) {
            return reference.clone();
        }

        let mut strings = self.string_consts.write().unwrap();

        let name_ref = self.insert_string_const_inner(
            runtime.method_area.insert(runtime.clone(), "java.lang.String").0.clone(),
            &mut strings, &mut values, class);

        let class_class_info = self.inner.add_class(class_class.clone());
        let class_obj = self.inner.allocator.new_object(class_class_info);

        let name_field = const_pool::Field {
            name: "name".to_string(),
            descriptor: FieldType::from_descriptor("Ljava/lang.String;").unwrap(),
            class: Arc::new(const_pool::Class { name: "java.lang.Class".to_string() }),
        };

        class_obj.set_field(&name_field, Value::Reference(name_ref));

        let class_ref = Heap::insert(&mut values, HeapValue::Object(Arc::new(class_obj)));
        classes.insert(class.to_string(), class_ref);
        class_ref
    }

    fn insert_string_const_inner(&self,
                                 string_class: Arc<Class>,
                                 strings: &mut RwLockWriteGuard<HashMap<String, Reference>>,
                                 values: &mut RwLockWriteGuard<HashMap<Reference, HeapValue>>,
                                 string: &str) -> Reference {
        if strings.contains_key(string) {
            return strings.get(string).unwrap().clone();
        }

        let utf16_chars: Vec<u16> = string.to_string().encode_utf16().collect();
        let arr = self.inner.allocator.new_array(ArrayType::Char, Int(utf16_chars.len() as i32));
        for (index, ch) in utf16_chars.iter().enumerate() {
            arr.set_element(Int(index as i32), Value::Int(Int(*ch as i32)));
        }

        let arr_ref = Heap::insert(values, HeapValue::Array(Arc::new(arr)));

        let string_class = self.inner.add_class(string_class);
        let string_obj = self.inner.allocator.new_object(string_class);

        let chars_field = const_pool::Field {
            name: "chars".to_string(),
            descriptor: FieldType::Array(Box::new(FieldType::Char)),
            class: Arc::new(const_pool::Class { name: "java.lang.String".to_string() }),
        };

        string_obj.set_field(&chars_field, Value::Reference(arr_ref));

        let obj_ref = Heap::insert(values, HeapValue::Object(Arc::new(string_obj)));

        strings.insert(string.to_string(), obj_ref);

        obj_ref
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
}

pub enum HeapValue {
    Array(Arc<Array>),
    Object(Arc<Object>),
}

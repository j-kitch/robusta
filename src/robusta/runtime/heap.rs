use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use rand::{RngCore, thread_rng};

use crate::java::{Int, Reference, Value};
use crate::runtime::method_area;
use crate::runtime::method_area::Class;

pub struct Heap {
    values: RwLock<HashMap<Reference, HeapValue>>,
    string_consts: RwLock<HashMap<String, Reference>>,
    class_objects: RwLock<HashMap<String, Reference>>
}

impl Heap {
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
        let char_arr = Arc::new(Array::Char(vec![0; length]));
        let mut values = self.values.write().unwrap();

        let arr_ref = Heap::insert(&mut values, HeapValue::Array(char_arr));

        arr_ref
    }

    pub fn insert_new(self: &Arc<Self>, class: &Arc<Class>) -> Reference {
        let fields: Vec<Arc<method_area::Field>> = class.inverse_hierarchy().iter()
            .flat_map(|class| class.fields.iter())
            .map(|field| field.clone())
            .collect();

        let obj = Object {
            class_name: class.name.clone(),
            fields: fields.iter().map(|f| {
                Arc::new(Field { value: f.descriptor.zero_value() })
            }).collect(),
        };

        println!("New obj {}", obj.class_name.as_str());

        // TODO: Extremely poor code here - very very temporary!
        let mut values = self.values.write().unwrap();

        Heap::insert(&mut values, HeapValue::Object(Arc::new(obj)))
    }

    pub fn get_class_object(self: &Arc<Self>, class: &str) -> Reference {
        let mut values = self.values.write().unwrap();
        let mut classes = self.class_objects.write().unwrap();

        if let Some(reference) = classes.get(class) {
            return reference.clone();
        }

        let mut strings = self.string_consts.write().unwrap();

        let name_ref = Heap::insert_string_const_inner(&mut strings, &mut values, class);
        let class_obj = Arc::new(Object {
            class_name: "java.lang.Class".to_string(),
            fields: vec![
                Arc::new(Field {
                    value: Value::Reference(name_ref)
                })
            ]
        });

        let class_ref = Heap::insert(&mut values, HeapValue::Object(class_obj));
        classes.insert(class.to_string(), class_ref);
        class_ref
    }

    fn insert_string_const_inner(strings: &mut RwLockWriteGuard<HashMap<String, Reference>>, values: &mut RwLockWriteGuard<HashMap<Reference, HeapValue>>, string: &str) -> Reference {
        if strings.contains_key(string) {
            return strings.get(string).unwrap().clone();
        }

        let bytes: Vec<u16> = string.to_string().encode_utf16().collect();
        let arr_ref = Heap::insert(values, HeapValue::Array(Arc::new(Array::Char(bytes))));

        let chars_field = Arc::new(Field { value: Value::Reference(Reference(0)) });
        chars_field.set_value(Value::Reference(arr_ref));

        let obj = HeapValue::Object(Arc::new(Object {
            class_name: "java.lang.String".to_string(),
            fields: vec![chars_field],
        }));
        let obj_ref = Heap::insert(values, obj);

        strings.insert(string.to_string(), obj_ref);

        obj_ref
    }

    pub fn insert_string_const(self: &Arc<Self>, string_const: &str) -> Reference {
        let mut string_consts = self.string_consts.write().unwrap();
        let mut values = self.values.write().unwrap();

        Heap::insert_string_const_inner(&mut string_consts, &mut values, string_const)
    }
}

pub enum HeapValue {
    Array(Arc<Array>),
    Object(Arc<Object>),
}

/// An array represented in the heap is one of the vectors defined in the enum.
pub enum Array {
    Char(Vec<u16>),
}

impl Array {
    pub fn length(&self) -> Int {
        match self {
            Array::Char(vec) => Int(vec.len() as i32)
        }
    }
}

/// An object represented in the heap is a reference to the class file and the field values.
pub struct Object {
    /// The class name is the key into the class.
    pub class_name: String,
    /// The set of fields, in definition order from superclass to leaf class.
    pub fields: Vec<Arc<Field>>,
}

pub struct Field {
    value: Value,
}

impl Field {
    pub fn get_value(&self) -> Value {
        self.value
    }

    /// This method uses unsafe rust to set the value without any multi-threading safety,
    /// to match the behaviour of the JVM.
    pub fn set_value(&self, value: Value) {
        unsafe {
            let v: *const Value = &self.value;
            let v = v as *mut Value;
            *v = value;
        }
    }
}
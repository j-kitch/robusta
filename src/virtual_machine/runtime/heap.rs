use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::java::{Double, Float, Int, Long, Reference, Value};

pub struct Heap {
    values: RwLock<HashMap<Reference, HeapValue>>,
    string_consts: RwLock<HashMap<String, Reference>>,
}

impl Heap {
    pub fn new() -> Arc<Self> {
        Arc::new(Heap {
            values: RwLock::new(HashMap::new()),
            string_consts: RwLock::new(HashMap::new()),
        })
    }

    pub fn insert_string_const(self: &Arc<Self>, string_const: &str) -> Reference {
        let mut string_consts = self.string_consts.write().unwrap();
        let mut values = self.values.write().unwrap();

        if string_consts.contains_key(string_const) {
            return string_consts.get(string_const).unwrap().clone();
        }

        let bytes: Vec<u16> = string_const.to_string().encode_utf16().collect();
        let arr_ref = Reference(values.len() as u32);

        values.insert(arr_ref, HeapValue::Array(Arc::new(Array::Char(bytes))));

        let obj_ref = Reference(values.len() as u32);
        let obj = HeapValue::Object(Arc::new(Object {
            class_name: "java.lang.String".to_string(),
            fields: vec![Arc::new(Field { value: Value::Reference(arr_ref) })],
        }));

        values.insert(obj_ref, obj);
        string_consts.insert(string_const.to_string(), obj_ref);

        obj_ref
    }
}

pub enum HeapValue {
    Array(Arc<Array>),
    Object(Arc<Object>),
}

/// An array represented in the heap is one of the vectors defined in the enum.
pub enum Array {
    Boolean(Vec<i8>),
    Byte(Vec<i8>),
    Char(Vec<u16>),
    Short(Vec<i16>),
    Int(Vec<Int>),
    Long(Vec<Long>),
    Float(Vec<Float>),
    Double(Vec<Double>),
    Reference(Vec<Reference>),
}

/// An object represented in the heap is a reference to the class file and the field values.
pub struct Object {
    /// The class name is the key into the class.
    class_name: String,
    /// The set of fields, in definition order from superclass to leaf class.
    fields: Vec<Arc<Field>>,
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
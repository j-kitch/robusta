use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::java::{Double, Float, Int, Long, Reference, Value};

pub struct Heap {
    values: RwLock<HashMap<Reference, HeapValue>>,
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
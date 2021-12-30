use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::class;
use crate::class::Class;
use crate::descriptor::Descriptor;
use crate::heap::Ref::Obj;

pub struct Heap {
    objects: HashMap<u32, Rc<RefCell<Ref>>>,
}

impl Heap {
    pub fn new() -> Self {
        Heap { objects: HashMap::new() }
    }

    pub fn insert_ref_array(&mut self, refs: Vec<u32>) -> u32 {
        let mut key: u32 = rand::random();
        while self.objects.contains_key(&key) {
            key = rand::random();
        }
        let key = key;

        self.objects.insert(key, Rc::from(RefCell::from(Ref::Arr(Array::Ref(refs)))));

        key
    }

    pub fn insert_char_array(&mut self, chars: Vec<u16>) -> u32 {
        let mut key: u32 = rand::random();
        while self.objects.contains_key(&key) {
            key = rand::random();
        }
        let key = key;

        self.objects.insert(key, Rc::from(RefCell::from(Ref::Arr(Array::Char(chars)))));

        key
    }

    pub fn create(&mut self, class: Rc<Class>) -> (u32, Rc<RefCell<Ref>>) {
        // TODO: Assuming not an array.
        let mut key: u32 = rand::random();
        while self.objects.contains_key(&key) {
            key = rand::random();
        }
        let key = key;

        let mut fields = vec![];
        class.for_each_field(|field| {
            let value = match &field.descriptor {
                Descriptor::Object(_) | Descriptor::Array(_) => Value::Ref(0),
                _ => panic!("Not implemented value of type {}", &field.descriptor)
            };
            fields.push(Field { field, value });
        });

        let object = Rc::from(RefCell::from(Obj(Object { class, fields })));
        self.objects.insert(key, object.clone());
        (key, object)
    }

    pub fn get(&self, key: u32) -> Rc<RefCell<Ref>> {
        self.objects.get(&key).unwrap().clone()
    }
}

pub enum Ref {
    Obj(Object),
    Arr(Array),
}

impl Ref {
    pub fn obj(&self) -> &Object {
        match self {
            Ref::Obj(obj) => obj,
            _ => panic!("err")
        }
    }

    pub fn arr(&self) -> &Array {
        match self {
            Ref::Arr(arr) => arr,
            _ => panic!("err")
        }
    }
}

pub struct Object {
    pub class: Rc<Class>,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub field: Rc<class::Field>,
    pub value: Value,
}

pub enum Value {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Ref(u32),
}

impl Value {
    pub fn reference(&self) -> u32 {
        match self {
            Value::Ref(u32) => u32.clone(),
            _ => panic!("err")
        }
    }

    pub fn int(&self) -> i32 {
        match self {
            Value::Int(i32) => i32.clone(),
            _ => panic!("err")
        }
    }

    pub fn long(&self) -> i64 {
        match self {
            Value::Long(i64) => i64.clone(),
            _ => panic!("err")
        }
    }

    pub fn float(&self) -> f32 {
        match self {
            Value::Float(f32) => f32.clone(),
            _ => panic!("err")
        }
    }

    pub fn double(&self) -> f64 {
        match self {
            Value::Double(f64) => f64.clone(),
            _ => panic!("err")
        }
    }
}

pub enum Array {
    Ref(Vec<u32>),
    Byte(Vec<i8>),
    Char(Vec<u16>),
}

impl Array {
    pub fn reference(&self) -> &Vec<u32> {
        match self {
            Array::Ref(vec) => vec,
            _ => panic!("err")
        }
    }

    pub fn byte(&self) -> &Vec<i8> {
        match self {
            Array::Byte(vec) => vec,
            _ => panic!("err")
        }
    }

    pub fn char(&self) -> &Vec<u16> {
        match self {
            Array::Char(vec) => vec,
            _ => panic!("err")
        }
    }
}

impl Array {
    pub fn len(&self) -> i32 {
        let len = match self {
            Array::Ref(v) => v.len(),
            Array::Byte(v) => v.len(),
            Array::Char(v) => v.len(),
        };
        len as i32
    }
}

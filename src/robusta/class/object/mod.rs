use std::rc::Rc;
use std::collections::HashMap;
use crate::descriptor::{Descriptor, MethodDescriptor};
use crate::robusta::class_file::Version;

#[derive(Debug)]
pub struct Class {
    pub version: Version,
    pub const_pool: HashMap<u16, Const>,
    pub access_flags: u16,
    pub this_class: std::string::String,
    pub super_class: Option<Rc<Class>>,
    pub interfaces: Vec<std::string::String>,
    pub fields: Vec<Rc<Field>>,
    pub methods: Vec<Rc<Method>>,
}

pub struct Iter<'a> {
    curr: Option<&'a Class>,
}

impl Class {
    pub fn const_method(&self, idx: u16) -> &MethodRef {
        match self.const_pool.get(&idx).unwrap() {
            Const::Method(method_ref) => method_ref,
            _ => panic!("err")
        }
    }

    pub fn for_each_field<F>(&self, f: F) where F: FnMut(Rc<Field>) {
        self.parent_iter()
            .flat_map(|class| class.fields.iter())
            .map(|f| f.clone())
            .for_each(f)
    }

    pub fn get_static_field_idx(&self, name: &str, descriptor: &Descriptor) -> Option<u16> {
        let idx = self.fields.iter()
            .enumerate()
            .find(|(_, f)| f.name.eq(name) && f.descriptor.eq(descriptor))
            .map(|(idx, _)| idx as u16);

        idx
    }

    pub fn find_method(&self, name: &str, descriptor: &MethodDescriptor) -> Option<Rc<Method>> {
        self.parent_iter()
            .flat_map(|class| class.methods.iter())
            .find(|method| method.name.eq(name) && method.descriptor.eq(descriptor))
            .map(|method| method.clone())
    }

    pub fn parent_iter(&self) -> Iter {
        Iter { curr: Some(self) }
    }

    pub fn is_instance_of(&self, descriptor: &Descriptor) -> bool {
        if let Descriptor::Object(name) = descriptor {
            self.parent_iter().any(|c| c.this_class.eq(name))
        } else {
            false
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Class;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.curr;

        self.curr = self.curr.and_then(|class| {
            class.super_class.as_ref().map(|class| class.as_ref())
        });

        val
    }
}

#[derive(Debug)]
pub enum Const {
    Class(ClassRef),
    Method(MethodRef),
    Field(FieldRef),
    Int(Integer),
    Float(Float),
    Double(Double),
    Long(Long),
    String(String),
}

#[derive(Debug)]
pub struct ClassRef {
    pub name: std::string::String,
}

#[derive(Debug)]
pub struct Integer {
    pub int: i32,
}

#[derive(Debug)]
pub struct Float {
    pub float: f32,
}

#[derive(Debug)]
pub struct Long {
    pub long: i64,
}

#[derive(Debug)]
pub struct Double {
    pub double: f64,
}

#[derive(Debug)]
pub struct String {
    pub string: std::string::String,
}

#[derive(Debug)]
pub struct FieldRef {
    pub class: std::string::String,
    pub name: std::string::String,
    pub descriptor: Descriptor,
}

#[derive(Clone, Debug)]
pub struct MethodRef {
    pub class: std::string::String,
    pub name: std::string::String,
    pub descriptor: MethodDescriptor,
}

#[derive(Debug)]
pub struct Field {
    pub name: std::string::String,
    pub descriptor: Descriptor,
    pub access_flags: u16,
}

#[derive(Debug)]
pub struct Method {
    pub name: std::string::String,
    pub descriptor: MethodDescriptor,
    pub native: bool,
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
}

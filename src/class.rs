use std::collections::HashMap;
use std::iter::{FlatMap, Map};
use std::rc::Rc;
use crate::class_file;
use crate::class_file::ClassFile;
use crate::descriptor::{Descriptor, MethodDescriptor};

#[derive(Debug)]
pub struct Class {
    pub minor_version: u16,
    pub major_version: u16,
    pub const_pool: HashMap<u16, Const>,
    pub access_flags: u16,
    pub this_class: String,
    pub super_class: Option<Rc<Class>>,
    pub interfaces: Vec<String>,
    pub fields: Vec<Rc<Field>>,
    pub methods: Vec<Rc<Method>>,
}

pub struct Iter<'a> {
    curr: Option<&'a Class>
}

type FieldIter<'a> = Map<FlatMap<Iter<'a>, std::slice::Iter<'a, Rc<Field>>, fn(&'a Class) -> std::slice::Iter<'a, Rc<Field>>>, fn(&Rc<Field>) -> Rc<Field>>;

impl Class {

    pub fn for_each_field<F>(&self, f: F) where F: FnMut(Rc<Field>) {
        self.parent_iter()
            .flat_map(|class| class.fields.iter())
            .map(|f| f.clone())
            .for_each(f)
    }

    pub fn find_method(&self, name: &str, descriptor: &MethodDescriptor) -> Option<Rc<Method>> {
        self.parent_iter()
            .flat_map(|class| class.methods.iter())
            .find(|method| method.name.eq(name) && method.descriptor.eq(descriptor))
            .map(|method| method.clone())
    }

    fn parent_iter(&self) -> Iter {
        Iter { curr: Some(self) }
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
}

#[derive(Debug)]
pub struct ClassRef {
    pub name: String,
}

#[derive(Debug)]
pub struct FieldRef {
    pub class: String,
    pub name: String,
    pub descriptor: Descriptor,
}

#[derive(Debug)]
pub struct MethodRef {
    pub class: String,
    pub name: String,
    pub descriptor: MethodDescriptor,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub descriptor: Descriptor,
}

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub descriptor: MethodDescriptor,
    pub native: bool,
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
}

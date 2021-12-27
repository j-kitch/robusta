use std::collections::HashMap;
use std::rc::Rc;
use crate::class_file;
use crate::class_file::ClassFile;

#[derive(Debug)]
pub struct Class {
    pub minor_version: u16,
    pub major_version: u16,
    pub const_pool: HashMap<u16, Const>,
    pub access_flags: u16,
    pub this_class: String,
    pub super_class: Option<Rc<Class>>,
    pub interfaces: Vec<String>,
    pub methods: Vec<Rc<Method>>,
}

pub struct Iter<'a> {
    curr: Option<&'a Class>
}

impl Class {
    pub fn find_method(&self, name: &str, descriptor: &str) -> Option<Rc<Method>> {
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
}

#[derive(Debug)]
pub struct ClassRef {
    pub name: String,
}

#[derive(Debug)]
pub struct MethodRef {
    pub class: String,
    pub name: String,
    pub descriptor: String,
}

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub descriptor: String,
    pub native: bool,
    pub code: Vec<u8>,
}

use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::ptr::NonNull;
use crate::class_file::ClassFile;

use crate::collection::once::{Once, OnceMap};
use crate::java::{FieldType, MethodType, Reference};
use crate::method_area::{Class, Field, Method};

/// The run-time constant pool of a class is a collection of constants and symbolic references to
/// other data in the JVM.
pub struct ConstPool {
    pool: HashMap<u16, Const>,
}

impl ConstPool {
    pub fn new(class_file: &ClassFile) -> Self {
        todo!()
    }

    pub fn get_const(&self, index: u16) -> &Const {
        self.pool.get(&index).unwrap()
    }

    pub fn get_class(&self, index: u16) -> &SymbolicReference<ClassKey, *const Class> {
        match self.pool.get(&index).unwrap() {
            Const::Class(reference) => reference,
            _ => panic!("Expected to find a class at index {} in the constant pool", index)
        }
    }

    pub fn get_method(&self, index: u16) -> &SymbolicReference<MethodKey, *const Method> {
        match self.pool.get(&index).unwrap() {
            Const::Method(reference) => reference,
            _ => panic!("Expected to find a method at index {} in the constant pool", index)
        }
    }

    pub fn get_field(&self, index: u16) -> &SymbolicReference<FieldKey, *const Field> {
        match self.pool.get(&index).unwrap() {
            Const::Field(reference) => reference,
            _ => panic!("Expected to find a field at index {} in the constant pool", index)
        }
    }
}

pub enum Const {
    Class(SymbolicReference<ClassKey, *const Class>),
    Field(SymbolicReference<FieldKey, *const Field>),
    Method(SymbolicReference<MethodKey, *const Method>),
    String(SymbolicReference<String, Reference>),
    Integer(i32),
}

pub struct ClassKey {
    pub name: String,
}

pub struct FieldKey {
    pub class: String,
    pub name: String,
    pub descriptor: FieldType,
}

pub struct MethodKey {
    pub class: String,
    pub name: String,
    pub descriptor: MethodType,
}

/// A symbolic reference is a resolvable reference to another object in the method area, or to a
/// java value.
pub struct SymbolicReference<K, V: Unpin> {
    const_key: K,
    resolved: Once<V>,
}

impl<K, V: Unpin> SymbolicReference<K, V> {
    pub fn resolve<F>(&self, f: F) -> &V
        where F: FnOnce(&K) -> V
    {
        self.resolved.get_or_init(|| f(&self.const_key))
    }
}


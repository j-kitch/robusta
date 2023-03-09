use std::collections::HashMap;

use crate::class_file::ClassFile;
use crate::class_file::const_pool as cp;
use crate::collection::once::Once;
use crate::java::{FieldType, MethodType, Reference};
use crate::method_area::{Class, Field, Method};

/// The run-time constant pool of a class is a collection of constants and symbolic references to
/// other data in the JVM.
pub struct ConstPool {
    pub pool: HashMap<u16, Const>,
}

unsafe impl Send for ConstPool {}

impl ConstPool {
    pub fn new(file: &ClassFile) -> Self {
        let pool: HashMap<u16, Const> = HashMap::new();
        let mut pool = ConstPool { pool };

        // Want to descend keys to ensure that when we visit references to other constants,
        // that those constants have already been added.
        let mut keys: Vec<(&u16, &crate::class_file::const_pool::Const)> = file.const_pool.iter().collect();
        keys.sort_by_key(|(_, con)| con.order());

        for (key, _) in keys {
            let val = file.const_pool.get(&key).unwrap();
            match val {
                cp::Const::Integer(integer) => {
                    pool.pool.insert(*key, Const::Integer(integer.int));
                }
                cp::Const::String(string) => {
                    let string = file.get_const_utf8(string.string);
                    let string = String::from_utf8(string.bytes.clone()).unwrap();
                    pool.pool.insert(*key, Const::String(SymbolicReference {
                        const_key: string.to_string(),
                        resolved: Once::new(),
                    }));
                }
                cp::Const::Class(class) => {
                    let name = file.get_const_utf8(class.name);
                    let name = String::from_utf8(name.bytes.clone()).unwrap()
                        .replace("/", ".");
                    pool.pool.insert(*key, Const::Class(SymbolicReference {
                        const_key: ClassKey { name: name },
                        resolved: Once::new(),
                    }));
                }
                cp::Const::FieldRef(field) => {
                    let class = file.get_const_class(field.class);
                    let class_name = file.get_const_utf8(class.name);
                    let name_and_type = file.get_const_name_and_type(field.name_and_type);
                    let name = file.get_const_utf8(name_and_type.name);
                    let name = String::from_utf8(name.bytes.clone()).unwrap();
                    let descriptor = file.get_const_utf8(name_and_type.descriptor);
                    let descriptor = FieldType::from_descriptor(String::from_utf8(descriptor.bytes.clone()).unwrap().as_str()).unwrap();
                    pool.pool.insert(*key, Const::Field(SymbolicReference {
                        const_key: FieldKey {
                            class: String::from_utf8(class_name.bytes.clone()).unwrap().replace("/", "."),
                            name,
                            descriptor,
                        },
                        resolved: Once::new(),
                    }));
                }
                cp::Const::MethodRef(method) => {
                    let class = file.get_const_class(method.class);
                    let class_name = file.get_const_utf8(class.name);
                    let name_and_type = file.get_const_name_and_type(method.name_and_type);
                    let name = file.get_const_utf8(name_and_type.name);
                    let name = String::from_utf8(name.bytes.clone()).unwrap();
                    let descriptor = file.get_const_utf8(name_and_type.descriptor);
                    let descriptor = MethodType::from_descriptor(String::from_utf8(descriptor.bytes.clone()).unwrap().as_str()).unwrap();
                    pool.pool.insert(*key, Const::Method(SymbolicReference {
                        const_key: MethodKey {
                            class: String::from_utf8(class_name.bytes.clone()).unwrap().replace("/", "."),
                            name,
                            descriptor,
                        },
                        resolved: Once::new(),
                    }));
                }
                _ => {}
            }
        }
        pool
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
    pub const_key: K,
    pub resolved: Once<V>,
}

impl<K, V: Unpin> SymbolicReference<K, V> {
    pub fn resolve<F>(&self, f: F) -> &V
        where F: FnOnce(&K) -> V
    {
        self.resolved.get_or_init(|| f(&self.const_key))
    }
}


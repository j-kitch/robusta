use std::collections::HashMap;
use std::sync::Arc;

use crate::class_file::ClassFile;
use crate::class_file::const_pool as cp;
use crate::java::{FieldType, Int, MethodType, Reference};
use crate::runtime::Runtime;

/// The runtime constant pool is a per type, runtime data structure that serves the purpose of
/// the symbol table in a conventional programming language.
pub struct ConstPool {
    pool: HashMap<u16, Const>,
}

impl ConstPool {
    pub fn new(file: &ClassFile, runtime: Arc<Runtime>) -> ConstPool {
        let pool: HashMap<u16, Const> = HashMap::new();
        let mut pool = ConstPool { pool };

        // Want to descend keys to ensure that when we visit references to other constants,
        // that those constants have already been added.
        let mut keys: Vec<(&u16, &cp::Const)> = file.const_pool.iter().collect();
        keys.sort_by_key(|(_, con)| con.order());

        for (key, _) in keys {
            let val = file.const_pool.get(&key).unwrap();
            match val {
                cp::Const::Integer(integer) => {
                    pool.pool.insert(*key, Const::Integer(Arc::new(Integer { int: Int(integer.int )})));
                }
                cp::Const::String(string) => {
                    let string = file.get_const_utf8(string.string);
                    let string = String::from_utf8(string.bytes.clone()).unwrap();
                    let reference = runtime.heap.insert_string_const(runtime.clone(), string.as_str());
                    pool.pool.insert(*key, Const::String(Arc::new(StringConst { string: reference })));
                }
                cp::Const::Class(class) => {
                    let name = file.get_const_utf8(class.name);
                    let name = String::from_utf8(name.bytes.clone()).unwrap()
                        .replace("/", ".");
                    pool.pool.insert(*key, Const::Class(Arc::new(Class { name })));
                }
                cp::Const::FieldRef(field) => {
                    let class = pool.get_class(field.class).clone();
                    let name_and_type = file.get_const_name_and_type(field.name_and_type);
                    let name = file.get_const_utf8(name_and_type.name);
                    let name = String::from_utf8(name.bytes.clone()).unwrap();
                    let descriptor = file.get_const_utf8(name_and_type.descriptor);
                    let descriptor = FieldType::from_descriptor(String::from_utf8(descriptor.bytes.clone()).unwrap().as_str()).unwrap();
                    pool.pool.insert(*key, Const::Field(Arc::new(Field { class, name, descriptor })));
                }
                cp::Const::MethodRef(method) => {
                    let class = pool.get_class(method.class).clone();
                    let name_and_type = file.get_const_name_and_type(method.name_and_type);
                    let name = file.get_const_utf8(name_and_type.name);
                    let name = String::from_utf8(name.bytes.clone()).unwrap();
                    let descriptor = file.get_const_utf8(name_and_type.descriptor);
                    let descriptor = MethodType::from_descriptor(String::from_utf8(descriptor.bytes.clone()).unwrap().as_str()).unwrap();
                    pool.pool.insert(*key, Const::Method(Arc::new(Method { class, name, descriptor })));
                }
                _ => {}
            }
        }
        pool
    }

    pub fn get_field(&self, idx: u16) -> Arc<Field> {
        match self.pool.get(&idx).unwrap() {
            Const::Field(field) => field.clone(),
            _ => panic!()
        }
    }

    pub fn get_method(&self, idx: u16) -> Arc<Method> {
        match self.pool.get(&idx).unwrap() {
            Const::Method(method) => method.clone(),
            _ => panic!()
        }
    }

    pub fn get_class(&self, idx: u16) -> Arc<Class> {
        match self.pool.get(&idx).unwrap() {
            Const::Class(class) => class.clone(),
            _ => panic!()
        }
    }

    pub fn get_const(&self, idx: u16) -> &Const {
        self.pool.get(&idx).unwrap()
    }

    pub fn len(&self) -> usize {
        self.pool.len()
    }
}

#[derive(Debug, PartialEq)]
/// A single constant in a constant pool.
pub enum Const {
    Class(Arc<Class>),
    Field(Arc<Field>),
    Method(Arc<Method>),
    Integer(Arc<Integer>),
    String(Arc<StringConst>),
}

#[derive(Debug, PartialEq)]
/// A symbolic link to a class or interface type.
pub struct Class {
    /// The binary name of the class or interface, note that this is different from the internal
    /// form used in class files, but the standard method seen in `Class.getName()`.
    ///
    /// For more information, see [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.2.1).
    pub name: String,
}

#[derive(Debug, PartialEq)]
/// A symbolic link to a field.
pub struct Field {
    /// The name of the field.
    pub name: String,
    /// The type of the field.
    pub descriptor: FieldType,
    /// The class that the field is defined on.
    pub class: Arc<Class>,
}

#[derive(Debug, PartialEq)]
/// A symbolic link to a method.
pub struct Method {
    /// The name of the method.
    pub name: String,
    /// The type of the method.
    pub descriptor: MethodType,
    /// The class that the method is defined on.
    pub class: Arc<Class>,
}

#[derive(Debug, PartialEq)]
/// A constant integer.
pub struct Integer {
    pub int: Int,
}

#[derive(Debug, PartialEq)]
/// A reference to a constant string.
pub struct StringConst {
    pub string: Reference,
}
//
// #[cfg(test)]
// mod tests {
//     use crate::loader::Loader;
//     use crate::runtime::Runtime;
//
//     use super::*;
//
//     #[test]
//     fn empty_main() {
//         let runtime = Runtime::new();
//         let class_file = runtime.loader.find("EmptyMain").unwrap();
//         let const_pool = ConstPool::new(&class_file, runtime.heap.clone());
//
//         assert_eq!(const_pool.len(), 3);
//         assert_eq!(const_pool.get_method(1), Arc::new(Method {
//             name: "<init>".to_string(),
//             descriptor: MethodType::from_descriptor("()V").unwrap(),
//             class: Arc::new(Class { name: "java.lang.Object".to_string() }),
//         }));
//         assert_eq!(const_pool.get_const(2), &Const::Class(Arc::new(Class { name: "java.lang.Object".to_string() })));
//         assert_eq!(const_pool.get_const(7), &Const::Class(Arc::new(Class { name: "EmptyMain".to_string() })));
//     }
// }
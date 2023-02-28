use std::collections::HashMap;
use std::sync::Arc;

use crate::class_file;
use crate::class_file::ClassFile;
use crate::java::MethodType;

/// The runtime constant pool is a per type, runtime data structure that serves the purpose of
/// the symbol table in a conventional programming language.
pub struct ConstPool {
    pool: HashMap<u16, Const>,
}

impl ConstPool {
    pub fn new(file: &ClassFile) -> ConstPool {
        let mut pool = HashMap::new();

        // Want to descend keys to ensure that when we visit references to other constants,
        // that those constants have already been added.
        let mut keys: Vec<u16> = file.const_pool.keys().map(|key| *key).collect();
        keys.sort_by(|a, b| a.cmp(b).reverse());

        for key in keys {
            let val = file.const_pool.get(&key).unwrap();
            match val {
                class_file::Const::Class { name } => {
                    let name_const = file.const_pool.get(name).unwrap();
                    let name = if let class_file::Const::Utf8 { bytes } = name_const {
                        String::from_utf8(bytes.clone()).unwrap()
                            .replace("/", ".")
                    } else {
                        panic!("err")
                    };

                    pool.insert(key, Const::Class(Arc::new(Class { name })));
                }
                class_file::Const::MethodRef { class, name_and_type } => {
                    let class = pool.get(class).unwrap();
                    if let Const::Class(class) = class {
                        let name_and_type = file.const_pool.get(name_and_type).unwrap();
                        if let class_file::Const::NameAndType { name, descriptor } = name_and_type {
                            let name_const = file.const_pool.get(name).unwrap();
                            let name = if let class_file::Const::Utf8 { bytes } = name_const {
                                String::from_utf8(bytes.clone()).unwrap()
                            } else {
                                panic!("err")
                            };
                            let descriptor = file.const_pool.get(descriptor).unwrap();
                            let descriptor = if let class_file::Const::Utf8 { bytes } = descriptor {
                                MethodType::from_descriptor(&String::from_utf8(bytes.clone()).unwrap()).unwrap()
                            } else {
                                panic!()
                            };

                            let method = Const::Method(Arc::new(Method {
                                name,
                                descriptor,
                                class: class.clone(),
                            }));

                            pool.insert(key, method);
                        } else {
                            panic!()
                        }
                    } else {
                        panic!()
                    }
                }
                _ => {}
            }
        }
        ConstPool { pool }
    }

    pub fn len(&self) -> usize {
        self.pool.len()
    }

    pub fn get_class(&self, idx: u16) -> Arc<Class> {
        match self.pool.get(&idx).unwrap() {
            Const::Class(class) => class.clone(),
            _ => panic!()
        }
    }

    pub fn get_method(&self, idx: u16) -> Arc<Method> {
        match self.pool.get(&idx).unwrap() {
            Const::Method(method) => method.clone(),
            _ => panic!()
        }
    }
}

/// A single constant in a constant pool.
pub enum Const {
    Class(Arc<Class>),
    Method(Arc<Method>),
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
/// A symbolic link to a method.
pub struct Method {
    /// The name of the method.
    pub name: String,
    /// The type of the method.
    pub descriptor: MethodType,
    /// The class that the method is defined on.
    pub class: Arc<Class>,
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::class_file::loader::Loader;

    use super::*;

    #[test]
    fn empty_main() {
        let mut loader = Loader::new(Path::new("./classes/EmptyMain.class")).unwrap();
        let class_file = loader.read_class_file().unwrap();

        let const_pool = ConstPool::new(&class_file);

        assert_eq!(const_pool.len(), 3);
        assert_eq!(const_pool.get_method(1), Arc::new(Method {
            name: "<init>".to_string(),
            descriptor: MethodType::from_descriptor("()V").unwrap(),
            class: Arc::new(Class { name: "java.lang.Object".to_string() }),
        }));
        assert_eq!(const_pool.get_class(2), Arc::new(Class { name: "java.lang.Object".to_string() }));
        assert_eq!(const_pool.get_class(7), Arc::new(Class { name: "EmptyMain".to_string() }));
    }
}
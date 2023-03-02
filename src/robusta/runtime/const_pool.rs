use std::collections::HashMap;
use std::sync::Arc;

use crate::class_file;
use crate::class_file::ClassFile;
use crate::java::{FieldType, Int, MethodType, Reference};
use crate::runtime::heap::Heap;

/// The runtime constant pool is a per type, runtime data structure that serves the purpose of
/// the symbol table in a conventional programming language.
pub struct ConstPool {
    pool: HashMap<u16, Const>,
}

impl ConstPool {
    pub fn new(file: &ClassFile, heap: Arc<Heap>) -> ConstPool {
        let mut pool: HashMap<u16, Const> = HashMap::new();

        // Want to descend keys to ensure that when we visit references to other constants,
        // that those constants have already been added.
        let mut keys: Vec<(&u16, &class_file::Const)> = file.const_pool.iter().collect();
        keys.sort_by_key(|(_, c)| c.runtime_pool_order());

        for (key, _) in keys {
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

                    pool.insert(*key, Const::Class(Arc::new(Class { name })));
                }
                class_file::Const::FieldRef { class, name_and_type } => {
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
                                FieldType::from_descriptor(&String::from_utf8(bytes.clone()).unwrap()).unwrap()
                            } else {
                                panic!()
                            };

                            let field = Const::Field(Arc::new(Field {
                                name,
                                descriptor,
                                class: class.clone(),
                            }));

                            pool.insert(*key, field);
                        } else {
                            panic!()
                        }
                    } else {
                        panic!()
                    }
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

                            pool.insert(*key, method);
                        } else {
                            panic!()
                        }
                    } else {
                        panic!()
                    }
                }
                class_file::Const::Integer { int } => {
                    pool.insert(*key, Const::Integer(Arc::new(Integer { int: Int(*int) })));
                }
                class_file::Const::String { string } => {
                    let string = if let class_file::Const::Utf8 { bytes } = file.const_pool.get(string).unwrap() {
                        let str = String::from_utf8(bytes.clone()).unwrap();
                        let str_ref = heap.insert_string_const(str.as_str());
                        str_ref
                    } else {
                        panic!()
                    };
                    pool.insert(*key, Const::String(Arc::new(StringConst { string })));
                }
                _ => {}
            }
        }
        ConstPool { pool }
    }

    pub fn get_method(&self, idx: u16) -> Arc<Method> {
        match self.pool.get(&idx).unwrap() {
            Const::Method(method) => method.clone(),
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::class_file::loader::Loader;

    use super::*;

    #[test]
    fn empty_main() {
        let heap = Heap::new();
        let mut loader = Loader::new(Path::new("./classes/EmptyMain.class")).unwrap();
        let class_file = loader.read_class_file().unwrap();

        let const_pool = ConstPool::new(&class_file, heap);

        assert_eq!(const_pool.len(), 3);
        assert_eq!(const_pool.get_method(1), Arc::new(Method {
            name: "<init>".to_string(),
            descriptor: MethodType::from_descriptor("()V").unwrap(),
            class: Arc::new(Class { name: "java.lang.Object".to_string() }),
        }));
        assert_eq!(const_pool.get_const(2), &Const::Class(Arc::new(Class { name: "java.lang.Object".to_string() })));
        assert_eq!(const_pool.get_const(7), &Const::Class(Arc::new(Class { name: "EmptyMain".to_string() })));
    }

    #[test]
    fn java_lang_string() {
        let heap = Heap::new();
        let mut loader = Loader::new(Path::new("./classes/java/lang/String.class")).unwrap();
        let class_file = loader.read_class_file().unwrap();

        let const_pool = ConstPool::new(&class_file, heap);

        assert_eq!(const_pool.len(), 4);
        assert_eq!(const_pool.get_method(1), Arc::new(Method {
            name: "<init>".to_string(),
            descriptor: MethodType::from_descriptor("()V").unwrap(),
            class: Arc::new(Class { name: "java.lang.Object".to_string() }),
        }));
        assert_eq!(const_pool.get_const(2), &Const::Field(Arc::new(Field {
            class: Arc::new(Class { name: "java.lang.String".to_string() }),
            name: "chars".to_string(),
            descriptor: FieldType::from_descriptor("[C").unwrap(),
        })));
        assert_eq!(const_pool.get_const(3), &Const::Class(Arc::new(Class { name: "java.lang.String".to_string() })));
        assert_eq!(const_pool.get_const(4), &Const::Class(Arc::new(Class { name: "java.lang.Object".to_string() })));
    }
}
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;

use crate::class;
use crate::class::Class;
use crate::class_file::{ClassFile, Reader};
use crate::class_file;
use crate::descriptor::{Descriptor, MethodDescriptor};

// TODO: This is extremely brittle!
const CLASS_PATH: &str = "/Users/joshkitc/personal/robusta/java";
const ACC_NATIVE: u16 = 0x0100;

pub struct ClassLoader {
    loaded: HashMap<String, Rc<Class>>,
}

impl ClassLoader {
    pub fn new() -> Self {
        ClassLoader { loaded: HashMap::new() }
    }

    pub fn load(&mut self, class: &str) -> Option<Rc<Class>> {
        if !self.loaded.contains_key(class) {
            let file_name = Path::new(CLASS_PATH)
                .join(class)
                .with_extension("class");

            let file = File::open(file_name);
            if file.is_err() {
                return None;
            }
            let file = file.unwrap();
            let mut reader = Reader::new(&file);
            let class_file = reader.read_class_file();
            let class = self.class_from(&class_file);
            self.loaded.insert(class.this_class.clone(), class);
        }
        self.loaded.get(class).map(|class| class.clone())
    }

    fn class_from(&mut self, class_file: &ClassFile) -> Rc<Class> {
        let mut const_pool = HashMap::new();
        for (idx, con) in class_file.const_pool.iter() {
            let con = match con {
                class_file::Const::Class(class) => {
                    let class_file::Utf8 { bytes } = class_file.get_const(class.name_idx).expect_utf8();
                    let name = String::from_utf8(bytes.clone()).unwrap();
                    class::Const::Class(class::ClassRef { name })
                }
                class_file::Const::Int(int) => {
                    class::Const::Int(class::Integer { int: int.int })
                }
                class_file::Const::Float(float) => {
                    class::Const::Float(class::Float { float: float.float })
                }
                class_file::Const::Long(long) => {
                    class::Const::Long(class::Long { long: long.long })
                }
                class_file::Const::FieldRef(field_ref) => {
                    let class = class_file.get_const(field_ref.class_idx).expect_class();
                    let class_name = class_file.get_const(class.name_idx).expect_utf8();

                    let name_and_type = class_file.get_const(field_ref.name_and_type_idx).expect_name_and_type();
                    let name = class_file.get_const(name_and_type.name_idx).expect_utf8();
                    let descriptor = class_file.get_const(name_and_type.descriptor_idx).expect_utf8();

                    let class_name = String::from_utf8(class_name.bytes.clone()).unwrap();
                    let name = String::from_utf8(name.bytes.clone()).unwrap();
                    let descriptor = String::from_utf8(descriptor.bytes.clone()).unwrap();

                    class::Const::Field(class::FieldRef { class: class_name, name, descriptor: Descriptor::parse(&descriptor) })
                }
                class_file::Const::MethodRef(method_ref) => {
                    let class = class_file.get_const(method_ref.class_idx).expect_class();
                    let class_name = class_file.get_const(class.name_idx).expect_utf8();

                    let name_and_type = class_file.get_const(method_ref.name_and_type_idx).expect_name_and_type();
                    let name = class_file.get_const(name_and_type.name_idx).expect_utf8();
                    let descriptor = class_file.get_const(name_and_type.descriptor_idx).expect_utf8();

                    let class_name = String::from_utf8(class_name.bytes.clone()).unwrap();
                    let name = String::from_utf8(name.bytes.clone()).unwrap();
                    let descriptor = String::from_utf8(descriptor.bytes.clone()).unwrap();

                    class::Const::Method(class::MethodRef { class: class_name, name, descriptor: MethodDescriptor::parse(&descriptor) })
                }
                _ => {
                    continue;
                }
            };
            const_pool.insert(idx.clone(), con);
        }

        let this_class = class_file.get_const(class_file.this_class).expect_class();
        let this_class_name = class_file.get_const(this_class.name_idx).expect_utf8();
        let this_class = String::from_utf8(this_class_name.bytes.clone()).unwrap();

        let super_class = Some(class_file.super_class)
            .filter(|idx| *idx != 0)
            .map(|idx| {
                let super_class = class_file.get_const(idx).expect_class();
                let super_class_name = class_file.get_const(super_class.name_idx).expect_utf8();
                let super_class_name = String::from_utf8(super_class_name.bytes.clone()).unwrap();
                self.load(&super_class_name).expect(&format!("Could not load class {}", &super_class_name))
            });

        let interfaces = class_file.interfaces.iter().map(|idx| {
            let interface = class_file.get_const(idx.clone()).expect_class();
            let interface_name = class_file.get_const(interface.name_idx).expect_utf8();
            String::from_utf8(interface_name.bytes.clone()).unwrap()
        }).collect();

        let fields = class_file.fields.iter().map(|field| {
            let name = class_file.get_const(field.name_idx).expect_utf8();
            let descriptor = class_file.get_const(field.descriptor_idx).expect_utf8();
            Rc::new(class::Field {
                name: String::from_utf8(name.bytes.clone()).unwrap(),
                descriptor: Descriptor::parse(
                    String::from_utf8(descriptor.bytes.clone()).unwrap().as_str()),
            })
        }).collect();

        let methods = class_file.methods.iter().map(|method| {
            let name = class_file.get_const(method.name_idx).expect_utf8();
            let descriptor = class_file.get_const(method.descriptor_idx).expect_utf8();
            let native = (method.access_flags & ACC_NATIVE) != 0;
            let mut max_locals = 0;
            let mut max_stack = 0;
            let code = if native { vec![] } else {
                let code = method.attributes.iter()
                    .find_map(|attr| match attr {
                        class_file::Attribute::Code(code) => Some(code),
                        _ => None
                    }).unwrap();
                max_locals = code.max_locals;
                max_stack = code.max_stack;
                code.code.clone()
            };
            Rc::new(class::Method {
                name: String::from_utf8(name.bytes.clone()).unwrap(),
                descriptor: MethodDescriptor::parse(
                    String::from_utf8(descriptor.bytes.clone()).unwrap().as_str()),
                native,
                max_locals,
                max_stack,
                code,
            })
        }).collect();

        Rc::from(Class {
            minor_version: class_file.minor_version,
            major_version: class_file.major_version,
            const_pool,
            access_flags: class_file.access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
        })
    }
}

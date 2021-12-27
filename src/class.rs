use std::collections::HashMap;
use crate::class_file;
use crate::class_file::ClassFile;

const ACC_NATIVE: u16 = 0x0100;

#[derive(Debug)]
pub struct Class {
    pub minor_version: u16,
    pub major_version: u16,
    pub const_pool: HashMap<u16, Const>,
    pub access_flags: u16,
    pub this_class: String,
    pub super_class: Option<String>,
    pub interfaces: Vec<String>,
    pub methods: Vec<Method>,
}

impl Class {

    pub fn from(class_file: &ClassFile) -> Class {
        let mut const_pool = HashMap::new();
        for (idx, con) in class_file.const_pool.iter().enumerate() {
            let con = match con {
                class_file::Const::Class(class) => {
                    let class_file::Utf8 { bytes } = class_file.get_const(class.name_idx).expect_utf8();
                    let name = String::from_utf8(bytes.clone()).unwrap();
                    Const::Class(ClassRef { name })
                },
                class_file::Const::MethodRef(method_ref) => {
                    let class = class_file.get_const(method_ref.class_idx).expect_class();
                    let class_name = class_file.get_const(class.name_idx).expect_utf8();

                    let name_and_type = class_file.get_const(method_ref.name_and_type_idx).expect_name_and_type();
                    let name = class_file.get_const(name_and_type.name_idx).expect_utf8();
                    let descriptor = class_file.get_const(name_and_type.descriptor_idx).expect_utf8();

                    let class_name = String::from_utf8(class_name.bytes.clone()).unwrap();
                    let name = String::from_utf8(name.bytes.clone()).unwrap();
                    let descriptor = String::from_utf8(descriptor.bytes.clone()).unwrap();

                    Const::Method(MethodRef { class: class_name, name, descriptor })
                },
                _ => {
                    continue
                }
            };
            let key = (idx + 1) as u16;
            const_pool.insert(key, con);
        }

        let this_class = class_file.get_const(class_file.this_class).expect_class();
        let this_class_name = class_file.get_const(this_class.name_idx).expect_utf8();
        let this_class = String::from_utf8(this_class_name.bytes.clone()).unwrap();

        let super_class = Some(class_file.super_class)
            .filter(|idx| *idx != 0)
            .map(|idx| {
                let super_class = class_file.get_const(idx).expect_class();
                let super_class_name = class_file.get_const(super_class.name_idx).expect_utf8();
                String::from_utf8(super_class_name.bytes.clone()).unwrap()
            });

        let interfaces = class_file.interfaces.iter().map(|idx| {
            let interface = class_file.get_const(idx.clone()).expect_class();
            let interface_name = class_file.get_const(interface.name_idx).expect_utf8();
            String::from_utf8(interface_name.bytes.clone()).unwrap()
        }).collect();

        let methods = class_file.methods.iter().map(|method| {
            let name = class_file.get_const(method.name_idx).expect_utf8();
            let descriptor = class_file.get_const(method.descriptor_idx).expect_utf8();
            let native = (method.access_flags & ACC_NATIVE) != 0;
            let code = if native { vec![] } else {
                let code = method.attributes.iter()
                    .find_map(|attr| match attr {
                        class_file::Attribute::Code(code) => Some(code),
                        _ => None
                    }).unwrap();
                code.code.clone()
            };
            Method {
                name: String::from_utf8(name.bytes.clone()).unwrap(),
                descriptor: String::from_utf8(descriptor.bytes.clone()).unwrap(),
                native,
                code,
            }
        }).collect();

        Class {
            minor_version: class_file.minor_version,
            major_version: class_file.major_version,
            const_pool,
            access_flags: class_file.access_flags,
            this_class,
            super_class,
            interfaces,
            methods,
        }
    }
}

#[derive(Debug)]
pub enum Const {
    Class(ClassRef),
    Method(MethodRef),
}

#[derive(Debug)]
pub struct ClassRef {
    name: String,
}

#[derive(Debug)]
pub struct MethodRef {
    class: String,
    name: String,
    descriptor: String,
}

#[derive(Debug)]
pub struct Method {
    name: String,
    descriptor: String,
    native: bool,
    code: Vec<u8>,
}

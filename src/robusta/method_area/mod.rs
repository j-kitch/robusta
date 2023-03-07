use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

use crate::class_file::{ACCESS_FLAG_NATIVE, ACCESS_FLAG_STATIC, Code};
use crate::collection::once::OnceMap;
use crate::java::{CategoryOne, CategoryTwo, FieldType, Int, MethodType, Reference};
use crate::loader::{ClassFileLoader, Loader};
use crate::method_area::const_pool::{Const, ConstPool, FieldKey, MethodKey};
use crate::runtime::heap::Heap;

pub mod const_pool;

struct MethodArea {
    loader: ClassFileLoader,
    classes: OnceMap<String, Class>,
}

impl MethodArea {
    pub fn resolve_category_one(&self, pool: NonNull<ConstPool>, index: u16) -> CategoryOne {
        let pool = unsafe { pool.as_ref() };
        match pool.get_const(index) {
            Const::Integer(int) => CategoryOne { int: Int(*int) },
            Const::String(reference) => {
                let reference = reference.resolve(|string| self.load_string(string));
                CategoryOne { reference: *reference }
            }
            Const::Class(reference) => {
                let class = reference.resolve(|key| self.load_class(&key.name));
                let class_object = self.load_class_object(unsafe { (*class).as_ref().unwrap() });
                CategoryOne { reference: class_object }
            }
            _ => panic!("Expected to load a category 1 const, but not found")
        }
    }

    pub fn resolve_category_two(&self, pool: NonNull<ConstPool>, index: u16) -> CategoryTwo {
        todo!()
    }

    /// Resolve a class symbolic reference in the constant pool, and return a reference to the
    /// class.
    pub fn resolve_class(&self, pool: NonNull<ConstPool>, index: u16) -> *const Class {
        let pool = unsafe { pool.as_ref() };
        let class_const = pool.get_class(index);
        let class = class_const.resolve(|class_key| {
            let class = self.load_class(&class_key.name);
            class as *const Class
        });
        *class
    }

    pub fn resolve_method(&self, pool: NonNull<ConstPool>, index: u16) -> *const Method {
        let pool = unsafe { pool.as_ref() };
        let method_const = pool.get_method(index);
        let method = method_const.resolve(|method_key| {
            let class = self.load_class(&method_key.class);
            let method = class.find_method(method_key);
            method as *const Method
        });
        *method
    }

    pub fn resolve_field(&self, pool: NonNull<ConstPool>, index: u16) -> *const Field {
        let pool = unsafe { pool.as_ref() };
        let field_const = pool.get_field(index);
        let field = field_const.resolve(|field_key| {
            let class = self.load_class(&field_key.class);
            let field = class.find_field(field_key);
            field as *const Field
        });
        *field
    }

    pub fn load_class(&self, class_name: &str) -> &Class {
        self.classes.get_or_init(class_name.to_string(), |name| {
            let class_file = self.loader.find(name).unwrap();
            let pool = ConstPool::new(&class_file);

            let super_class = if class_file.super_class == 0 { None } else {
                let super_class = pool.get_class(class_file.super_class);
                let super_class = super_class.resolve(|key| self.load_class(&key.name));
                Some(*super_class)
            };

            let mut fields: Vec<Field> = class_file.fields.iter()
                .map(|f| {
                    let is_static = (ACCESS_FLAG_STATIC & f.access_flags) != 0;
                    let name = class_file.get_const_utf8(f.name);
                    let name = String::from_utf8(name.bytes.clone()).unwrap();
                    let descriptor = class_file.get_const_utf8(f.descriptor);
                    let descriptor = FieldType::from_descriptor(String::from_utf8(descriptor.bytes.clone()).unwrap().as_str()).unwrap();
                    Field { is_static, name, width: descriptor.width(), descriptor, offset: 0 }
                }).collect();

            // Sort to get a better order for object packing.
            fields.sort_by(|a, b| a.width.cmp(&b.width).reverse());
            let mut offset = 0;
            for field in &mut fields {
                field.offset = offset;
                offset += field.width;
            }

            const ALIGN: usize = 4;

            // Get our final padded width.
            let padding = ALIGN - (offset % ALIGN);
            let width = offset + padding;

            let methods: Vec<Method> = class_file.methods.iter()
                .map(|m| {
                    let is_static = (m.access_flags & ACCESS_FLAG_STATIC) != 0;
                    let is_native = (m.access_flags & ACCESS_FLAG_NATIVE) != 0;
                    let name = class_file.get_const_utf8(m.name);
                    let name = String::from_utf8(name.bytes.clone()).unwrap();

                    let descriptor = class_file.get_const_utf8(m.descriptor);
                    let descriptor = MethodType::from_descriptor(String::from_utf8(descriptor.bytes.clone()).unwrap().as_str()).unwrap();
                    Method { is_static, is_native, name, descriptor, code: m.code.clone() }
                }).collect();

            let class = Class {
                name: name.to_string(),
                flags: ClassFlags { bits: class_file.access_flags },
                const_pool: pool,
                super_class,
                interfaces: vec![], // TODO: Implement
                fields,
                methods,
                attributes: vec![], // TODO: Implement,
                width
            };
            class
        })
    }

    pub fn load_string(&self, string: &str) -> Reference {
        todo!()
    }

    pub fn load_class_object(&self, class: &Class) -> Reference {
        todo!()
    }
}

pub struct Class {
    pub name: String,
    pub flags: ClassFlags,
    pub const_pool: ConstPool,
    pub super_class: Option<*const Class>,
    pub interfaces: Vec<*const Class>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
    pub width: usize,
}

pub struct Hierarchy {
    current: Option<*const Class>,
}

impl Iterator for Hierarchy {
    type Item = *const Class;

    fn next(&mut self) -> Option<Self::Item> {
        let class = self.current;
        self.current = self.current.and_then(|class| unsafe { (*class).super_class });
        class
    }
}

impl Class {
    fn parents(&self) -> Hierarchy {
        Hierarchy { current: Some(self as *const Class) }
    }

    pub fn find_field(&self, key: &FieldKey) -> &Field {
        self.parents()
            .flat_map(|class| unsafe { (*class).fields.iter() })
            .find(|fld| fld.name.eq(&key.name) && fld.descriptor.eq(&key.descriptor))
            .unwrap()
    }

    pub fn find_method(&self, key: &MethodKey) -> &Method {
        self.parents()
            .flat_map(|class| unsafe { (*class).methods.iter() })
            .find(|mthd| mthd.name.eq(&key.name) && mthd.descriptor.eq(&key.descriptor))
            .unwrap()
    }
}

pub struct ClassFlags {
    bits: u16,
}

pub struct Field {
    pub is_static: bool,
    pub name: String,
    pub descriptor: FieldType,
    pub offset: usize,
    pub width: usize,
}

pub struct Method {
    pub is_static: bool,
    pub is_native: bool,
    pub name: String,
    pub descriptor: MethodType,
    pub code: Option<Code>,
}

pub struct Attribute {}
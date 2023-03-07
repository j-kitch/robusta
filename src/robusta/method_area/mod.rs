use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

use crate::collection::once::OnceMap;
use crate::java::{CategoryOne, CategoryTwo, FieldType, Int, MethodType, Reference};
use crate::loader::{ClassFileLoader, Loader};
use crate::method_area::const_pool::{Const, ConstPool, FieldKey, MethodKey};
use crate::runtime::heap::Heap;

mod const_pool;

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
        todo!()
    }

    pub fn load_string(&self, string: &str) -> Reference {
        todo!()
    }

    pub fn load_class_object(&self, class: &Class) -> Reference {
        todo!()
    }
}

struct Class {
    name: String,
    flags: ClassFlags,
    const_pool: ConstPool,
    super_class: Option<Pin<NonNull<Class>>>,
    interfaces: Vec<Pin<NonNull<Class>>>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    attributes: Vec<Attribute>,
}

impl Class {
    fn find_field(&self, key: &FieldKey) -> &Field {
        todo!()
    }

    fn find_method(&self, key: &MethodKey) -> &Method {
        todo!()
    }
}

struct ClassFlags {
    bits: u16,
}

struct Field {
    pub name: String,
    pub descriptor: FieldType,
}

struct Method {
    name: String,
    descriptor: MethodType,
}

struct Attribute {}
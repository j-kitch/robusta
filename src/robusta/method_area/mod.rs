use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

use maplit::hashset;
use tracing::{debug, info};

use crate::class_file::{ACCESS_FLAG_NATIVE, ACCESS_FLAG_STATIC, ClassAttribute, Code, METHOD_ACC_SYNC};
use crate::collection::classes::{Classes, ClassRef};
use crate::collection::once::OnceMap;
use crate::heap::Heap;
use crate::java::{Double, FieldType, Float, Int, Long, MethodType, Reference, Value};
use crate::loader::{ClassFileLoader, Loader};
use crate::log;
use crate::method_area::const_pool::{Const, ConstPool, FieldKey, MethodKey};
use crate::runtime::Runtime;
use crate::thread::Thread;

pub mod const_pool;

pub struct MethodArea {
    loader: ClassFileLoader,
    heap: *const Heap,
    classes: Classes,
}

unsafe impl Send for MethodArea {}

unsafe impl Sync for MethodArea {}

impl MethodArea {
    pub fn new(heap: *const Heap) -> Self {
        MethodArea {
            loader: ClassFileLoader::new(vec![
                PathBuf::from("./classes"),
                PathBuf::from("./classes/rt.jar"),
            ]),
            heap,
            classes: Classes::new(),
        }
    }

    pub fn load_primitive_classes(&self) {
        self.classes.load_class("boolean", |_| {
            Class {
                name: "boolean".to_string(),
                flags: ClassFlags { bits: 0 },
                const_pool: ConstPool { pool: HashMap::new() },
                super_class: None,
                interfaces: vec![],
                instance_fields: vec![],
                static_fields: vec![],
                methods: vec![],
                attributes: vec![],
                instance_width: 0,
                static_width: 0,
                source_file: None,
            }
        });
        self.classes.load_class("byte", |_| {
            Class {
                name: "byte".to_string(),
                flags: ClassFlags { bits: 0 },
                const_pool: ConstPool { pool: HashMap::new() },
                super_class: None,
                interfaces: vec![],
                instance_fields: vec![],
                static_fields: vec![],
                methods: vec![],
                attributes: vec![],
                instance_width: 0,
                static_width: 0,
                source_file: None,
            }
        });
        self.classes.load_class("char", |_| {
            Class {
                name: "char".to_string(),
                flags: ClassFlags { bits: 0 },
                const_pool: ConstPool { pool: HashMap::new() },
                super_class: None,
                interfaces: vec![],
                instance_fields: vec![],
                static_fields: vec![],
                methods: vec![],
                attributes: vec![],
                instance_width: 0,
                static_width: 0,
                source_file: None,
            }
        });
        self.classes.load_class("short", |_| {
            Class {
                name: "short".to_string(),
                flags: ClassFlags { bits: 0 },
                const_pool: ConstPool { pool: HashMap::new() },
                super_class: None,
                interfaces: vec![],
                instance_fields: vec![],
                static_fields: vec![],
                methods: vec![],
                attributes: vec![],
                instance_width: 0,
                static_width: 0,
                source_file: None,
            }
        });
        self.classes.load_class("int", |_| {
            Class {
                name: "int".to_string(),
                flags: ClassFlags { bits: 0 },
                const_pool: ConstPool { pool: HashMap::new() },
                super_class: None,
                interfaces: vec![],
                instance_fields: vec![],
                static_fields: vec![],
                methods: vec![],
                attributes: vec![],
                instance_width: 0,
                static_width: 0,
                source_file: None,
            }
        });
        self.classes.load_class("float", |_| {
            Class {
                name: "float".to_string(),
                flags: ClassFlags { bits: 0 },
                const_pool: ConstPool { pool: HashMap::new() },
                super_class: None,
                interfaces: vec![],
                instance_fields: vec![],
                static_fields: vec![],
                methods: vec![],
                attributes: vec![],
                instance_width: 0,
                static_width: 0,
                source_file: None,
            }
        });
        self.classes.load_class("long", |_| {
            Class {
                name: "long".to_string(),
                flags: ClassFlags { bits: 0 },
                const_pool: ConstPool { pool: HashMap::new() },
                super_class: None,
                interfaces: vec![],
                instance_fields: vec![],
                static_fields: vec![],
                methods: vec![],
                attributes: vec![],
                instance_width: 0,
                static_width: 0,
                source_file: None,
            }
        });
        self.classes.load_class("double", |_| {
            Class {
                name: "double".to_string(),
                flags: ClassFlags { bits: 0 },
                const_pool: ConstPool { pool: HashMap::new() },
                super_class: None,
                interfaces: vec![],
                instance_fields: vec![],
                static_fields: vec![],
                methods: vec![],
                attributes: vec![],
                instance_width: 0,
                static_width: 0,
                source_file: None,
            }
        });
    }

    pub fn insert_gen_class(&self, class: Class) -> *const Class {
        let class = self.classes.load_class(&class.name.clone(), |_| class);
        class.self_referential();
        &*class as *const Class
    }

    pub fn resolve_category_one(&self, pool: *const ConstPool, index: u16) -> Value {
        let pool = unsafe { pool.as_ref().unwrap() };
        match pool.get_const(index) {
            Const::Integer(int) => Value::Int(Int(*int)),
            Const::Float(float) => Value::Float(Float(*float)),
            Const::String(reference) => {
                let reference = reference.resolve(|string| self.load_string(string));
                Value::Reference(*reference)
            }
            Const::Class(reference) => {
                let class = reference.resolve(|key| &*self.load_class(&key.name));
                let class_object = self.load_class_object(unsafe { (*class).as_ref().unwrap() });
                Value::Reference(class_object)
            }
            _ => panic!("Expected to load a category 1 const, but not found")
        }
    }

    pub fn resolve_category_two(&self, pool: *const ConstPool, index: u16) -> Value {
        let pool = unsafe { pool.as_ref().unwrap() };
        match pool.get_const(index) {
            Const::Long(long) => Value::Long(Long(*long)),
            Const::Double(double) => Value::Double(Double(*double)),
            _ => panic!("Expected to load a category 2 const, but not found")
        }
    }

    /// Resolve a class symbolic reference in the constant pool, and return a reference to the
    /// class.
    pub fn resolve_class(&self, pool: *const ConstPool, index: u16) -> *const Class {
        let pool = unsafe { pool.as_ref().unwrap() };
        let class_const = pool.get_class(index);
        let class = class_const.resolve(|class_key| {
            let class = self.load_class(&class_key.name);
            &*class as *const Class
        });
        *class
    }

    pub fn resolve_method(&self, pool: *const ConstPool, index: u16) -> *const Method {
        let pool = unsafe { pool.as_ref().unwrap() };
        let method_const = pool.get_method(index);
        let method = method_const.resolve(|method_key| {
        debug!("Resolving method {} {} {}", &method_key.class, &method_key.name, method_key.descriptor.descriptor());
            let class = self.load_class(&method_key.class);
            let method = class.find_method(method_key).unwrap();
        debug!("Resolved method {} {} {}", &method_key.class, &method_key.name, method_key.descriptor.descriptor());
            method as *const Method
        });
        debug!("Returning resolved method");
        *method
    }

    pub fn resolve_field(&self, _: Arc<Runtime>, pool: *const ConstPool, index: u16) -> *const Field {
        let pool = unsafe { pool.as_ref().unwrap() };
        let field_const = pool.get_field(index);
        let field = field_const.resolve(|field_key| {
            let class = self.load_class(&field_key.class);
            let field = class.find_field(field_key);
            field as *const Field
        });
        *field
    }

    pub fn resolve_static(&self, _: Arc<Runtime>, pool: *const ConstPool, index: u16) -> *const Field {
        let pool = unsafe { pool.as_ref().unwrap() };
        let field_const = pool.get_field(index);
        let field = field_const.resolve(|field_key| {
            let class = self.load_class(&field_key.class);
            let field = class.find_static(field_key);
            field as *const Field
        });
        *field
    }

    pub fn load_class(&self, class_name: &str) -> ClassRef {
        info!("loading class {}", class_name);
        let class = self.classes.load_class(class_name, |name| {
            debug!(target: log::LOADER, class=name, "Loading class");
            let class_file = self.loader.find(name).unwrap();
            let pool = ConstPool::new(&class_file);

            info!("F");
            let super_class = if class_file.super_class == 0 { None } else {
                let super_class = pool.get_class(class_file.super_class);
                let super_class = super_class.resolve(|key| &*self.load_class(&key.name));
                Some(*super_class)
            };

            let interfaces: Vec<*const Class> = class_file.interfaces.iter().map(|index| {
                let name = pool.get_class(*index);
                let class = name.resolve(|key| &*self.load_class(&key.name));
                *class
            }).collect();

            let mut instance_fields: Vec<Field> = class_file.fields.iter()
                .filter(|f| (f.access_flags & ACCESS_FLAG_STATIC) == 0)
                .map(|f| {
                    let is_static = false;
                    let name = class_file.get_const_utf8(f.name);
                    let name = String::from_utf8(name.bytes.clone()).unwrap();
                    let descriptor = class_file.get_const_utf8(f.descriptor);
                    let descriptor = FieldType::from_descriptor(String::from_utf8(descriptor.bytes.clone()).unwrap().as_str()).unwrap();
                    Field { class: 0 as *const Class, is_static, name, width: descriptor.width(), descriptor, offset: 0 }
                }).collect();

            // Sort to get a better order for object packing.
            instance_fields.sort_by(|a, b| a.width.cmp(&b.width).reverse());
            let mut instance_offset = unsafe { super_class.map_or(0, |c| (*c).instance_width) };
            for field in &mut instance_fields {
                field.offset = instance_offset;
                instance_offset += field.width;
            }

            let mut static_fields: Vec<Field> = class_file.fields.iter()
                .filter(|f| (f.access_flags & ACCESS_FLAG_STATIC) != 0)
                .map(|f| {
                    let is_static = true;
                    let name = class_file.get_const_utf8(f.name);
                    let name = String::from_utf8(name.bytes.clone()).unwrap();
                    let descriptor = class_file.get_const_utf8(f.descriptor);
                    let descriptor = FieldType::from_descriptor(String::from_utf8(descriptor.bytes.clone()).unwrap().as_str()).unwrap();
                    Field { class: 0 as *const Class, is_static, name, width: descriptor.width(), descriptor, offset: 0 }
                }).collect();

            // Sort to get a better order for object packing.
            static_fields.sort_by(|a, b| a.width.cmp(&b.width).reverse());
            let mut static_offset = 0; // parent fields arent included.
            for field in &mut static_fields {
                field.offset = static_offset;
                static_offset += field.width;
            }

            const ALIGN: usize = 4;

            // Get our final padded width.
            let instance_pad = ALIGN - (instance_offset % ALIGN);
            let instance_width = instance_offset + instance_pad;

            let static_pad = ALIGN - (static_offset % ALIGN);
            let static_width = static_offset + static_pad;

            let methods: Vec<Method> = class_file.methods.iter()
                .map(|m| {
                    let is_static = (m.access_flags & ACCESS_FLAG_STATIC) != 0;
                    let is_native = (m.access_flags & ACCESS_FLAG_NATIVE) != 0;
                    let is_synchronized = (m.access_flags & METHOD_ACC_SYNC) != 0;
                    let name = class_file.get_const_utf8(m.name);
                    let name = String::from_utf8(name.bytes.clone()).unwrap();

                    let descriptor = class_file.get_const_utf8(m.descriptor);
                    let descriptor = MethodType::from_descriptor(String::from_utf8(descriptor.bytes.clone()).unwrap().as_str()).unwrap();
                    Method { class: 0 as *const Class, is_static, is_native, is_synchronized, name, descriptor, code: m.code().map(|c| c.clone()) }
                }).collect();

            let source_file = class_file.attributes.iter()
                .find_map(|attr| {
                    match attr {
                        ClassAttribute::SourceFile(source_file) => {
                            let file_name = class_file.get_const_utf8(source_file.source_file);
                            let file_name = String::from_utf8(file_name.bytes.clone()).unwrap();
                            Some(file_name)
                        }
                        _ => None
                    }
                });

            let class = Class {
                name: name.to_string(),
                flags: ClassFlags { bits: class_file.access_flags },
                const_pool: pool,
                super_class,
                interfaces,
                instance_fields,
                static_fields,
                methods,
                attributes: vec![], // TODO: Implement,
                instance_width,
                static_width,
                source_file,
            };
            debug!(target: log::LOADER, class=name, "Loaded class");
            class
        });
        info!("A");
        class.self_referential();
        info!("B");
        if !class.static_fields.is_empty() {
            let heap = unsafe { self.heap.as_ref().unwrap() };
            heap.get_static(&*class);
        }
        info!("C");
        info!("loaded class {}", class_name);
        class
    }

    pub fn load_string(&self, string: &str) -> Reference {
        let heap = unsafe { self.heap.as_ref().unwrap() };
        let string_class = self.load_class("java.lang.String");
        heap.insert_string_const(string, &*string_class)
    }

    pub fn load_class_object(&self, class: &Class) -> Reference {
        let heap = unsafe { self.heap.as_ref().unwrap() };
        let string_class = self.load_class("java.lang.String");
        let class_class = self.load_class("java.lang.Class");
        heap.insert_class_object(class, &*class_class, &*string_class)
    }

    pub fn initialize(&self, thread: &mut Thread, class: &Class) {
        let already_init = thread.stack.iter().any(|f| {
            if f.method == 0 as *const Method {
                return false;
            }
            let method = unsafe { f.method.as_ref().unwrap() };
            method.name.eq("<clinit>") && f.class.eq(&class.name)
        });
        if already_init {
            return;
        }

        debug!("About to try to initialize class {}", &class.name);
        self.classes.initialize(&class.name, |_| {
        debug!("Starting initialize class {}", &class.name);
            if let Some(parent) = class.super_class {
                let parent = unsafe { parent.as_ref().unwrap() };
                self.initialize(thread, parent);
            }
            if let Some(clinit) = class.methods.iter().find(|m| m.name.eq("<clinit>")) {
                let depth = thread.stack.len();
                thread.push_frame(
                    class.name.clone(),
                    &class.const_pool as *const ConstPool,
                    clinit as *const Method,
                    vec![],
                );
                while thread.stack.len() > depth {
                    thread.next();
                }
            }
            debug!("Finished initialize class {}", &class.name);
        });
    }
}

pub struct Class {
    pub name: String,
    pub flags: ClassFlags,
    pub const_pool: ConstPool,
    pub super_class: Option<*const Class>,
    pub interfaces: Vec<*const Class>,
    pub instance_fields: Vec<Field>,
    pub static_fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
    pub instance_width: usize,
    pub static_width: usize,
    pub source_file: Option<String>,
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
    fn self_referential(&self) {
        let class = self as *const Class;
        for field in &self.instance_fields {
            let mut_ptr = (field as *const Field).cast_mut();
            unsafe {
                (*mut_ptr).class = class;
            }
        }
        for field in &self.static_fields {
            let mut_ptr = (field as *const Field).cast_mut();
            unsafe {
                (*mut_ptr).class = class;
            }
        }
        for method in &self.methods {
            let mut_ptr = (method as *const Method).cast_mut();
            unsafe {
                (*mut_ptr).class = class;
            }
        }
    }

    pub fn parents(&self) -> Hierarchy {
        Hierarchy { current: Some(self as *const Class) }
    }

    pub fn parents_and_interfaces(&self) -> HashSet<*const Class> {
        let mut classes = hashset! {};
        let mut visited = hashset! {};

        classes.insert(self as *const Class);

        while visited.len() < classes.len() {
            let next_class = classes.difference(&visited).next().unwrap().clone();
            let class = unsafe { next_class.as_ref().unwrap() };

            if let Some(parent) = class.super_class {
                classes.insert(parent);
                for interface in &class.interfaces {
                    classes.insert(*interface);
                }
            }

            visited.insert(next_class);
        }

        classes
    }

    pub fn find_field(&self, key: &FieldKey) -> &Field {
        self.parents()
            .flat_map(|class| unsafe { (*class).instance_fields.iter() })
            .find(|fld| fld.name.eq(&key.name) && fld.descriptor.eq(&key.descriptor))
            .unwrap()
    }

    pub fn find_static(&self, key: &FieldKey) -> &Field {
        self.static_fields.iter()
            .find(|f| f.name.eq(&key.name) && f.descriptor.eq(&key.descriptor))
            .unwrap()
    }

    pub fn find_method(&self, key: &MethodKey) -> Option<&Method> {
        self.parents()
            .flat_map(|class| unsafe { (*class).methods.iter() })
            .find(|mthd| mthd.name.eq(&key.name) && mthd.descriptor.eq(&key.descriptor))
    }

    pub fn is_instance_of(&self, other: &Class) -> bool {
        self.parents_and_interfaces().iter().any(|c| {
            let c = unsafe { c.as_ref().unwrap() };
            c.name.eq(&other.name)
        })
    }
}

pub struct ClassFlags {
    pub bits: u16,
}

pub struct Field {
    pub class: *const Class,
    pub is_static: bool,
    pub name: String,
    pub descriptor: FieldType,
    pub offset: usize,
    pub width: usize,
}

pub struct Method {
    pub class: *const Class,
    pub is_static: bool,
    pub is_native: bool,
    pub is_synchronized: bool,
    pub name: String,
    pub descriptor: MethodType,
    pub code: Option<Code>,
}

pub struct Attribute {}
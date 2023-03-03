use std::sync::Arc;
use std::sync::mpsc::SyncSender;

use crate::class_file::{ACCESS_FLAG_NATIVE, ACCESS_FLAG_STATIC, Code};
use crate::collection::AppendMap;
use crate::java::{FieldType, MethodType};
use crate::loader::Loader;
use crate::runtime::{const_pool, ConstPool, Runtime};

pub struct MethodArea {
    map: Arc<AppendMap<String, Class>>,
    /// TODO: Potentially could do with an append set?
    initialized: Arc<AppendMap<String, ()>>,
    resolved: Arc<AppendMap<String, ()>>,
}

impl MethodArea {
    pub fn new() -> Arc<Self> {
        Arc::new(MethodArea {
            map: AppendMap::new(),
            initialized: AppendMap::new(),
            resolved: AppendMap::new(),
        })
    }

    pub fn insert(self: &Arc<Self>, runtime: Arc<Runtime>, name: &str) -> (Arc<Class>, bool) {
        if name.contains('/') {
            panic!("MethodArea:Insert {}", name);
        }
        self.map.clone().get_or_insert(&name.to_string(), || {
            let class_file = runtime.loader.find(name).unwrap();
            let pool = ConstPool::new(&class_file, runtime.heap.clone());

            let super_class = if class_file.super_class == 0 { None } else {
                let super_class = pool.get_class(class_file.super_class);
                let (super_class, _) = self.insert(runtime.clone(), super_class.name.as_str());
                Some(super_class)
            };

            let fields: Vec<Arc<Field>> = class_file.fields.iter()
                .map(|f| {
                    let is_static = (ACCESS_FLAG_STATIC & f.access_flags) != 0;
                    let name = class_file.get_const_utf8(f.name);
                    let name = String::from_utf8(name.bytes.clone()).unwrap();
                    let descriptor = class_file.get_const_utf8(f.descriptor);
                    let descriptor = FieldType::from_descriptor(String::from_utf8(descriptor.bytes.clone()).unwrap().as_str()).unwrap();
                    Arc::new(Field { is_static, name, descriptor })
                }).collect();

            let mut is_initialised = true;

            let methods: Vec<Arc<Method>> = class_file.methods.iter()
                .map(|m| {
                    let is_static = (m.access_flags & ACCESS_FLAG_STATIC) != 0;
                    let is_native = (m.access_flags & ACCESS_FLAG_NATIVE) != 0;
                    let name = class_file.get_const_utf8(m.name);
                    let name = String::from_utf8(name.bytes.clone()).unwrap();

                    if name.eq("<clinit>") {
                        is_initialised = false;
                    }

                    let descriptor = class_file.get_const_utf8(m.descriptor);
                    let descriptor = MethodType::from_descriptor(String::from_utf8(descriptor.bytes.clone()).unwrap().as_str()).unwrap();
                    Arc::new(Method { is_static, is_native, name, descriptor, code: m.code.clone() })
                }).collect();

            if is_initialised {
                self.initialized.get_or_insert(&name.to_string(), || ());
            }

            Class {
                name: name.to_string(),
                super_class,
                const_pool: Arc::new(pool),
                fields,
                methods,
            }
        })
    }

    /// When we start trying to call <clinit>, we need to call each in order of superclasses.
    ///
    /// This method lets the calling thread know which classes it is in charge of loading.
    pub fn try_start_init(self: &Arc<Self>, class_name: &str) -> Vec<(Arc<Class>, SyncSender<()>)> {

        let mut class = self.map.get(&class_name.to_string());
        let mut classes = Vec::new();

        while let Some(curr_class) = class.as_ref() {
            classes.push(curr_class.clone());
            class = curr_class.super_class.clone().and_then(|c| self.map.get(&c.name));
        }

        let mut result = Vec::new();
        for class in classes {
            let sender = self.try_start_initialize(class.name.as_str());
            if sender.is_some() {
                result.push((class.clone(), sender.unwrap()));
            }
        }

        result
    }

    pub fn try_start_initialize(self: &Arc<Self>, class_name: &str) -> Option<SyncSender<()>> {
        self.initialized.begin_insert(&class_name.to_string())
    }

    pub fn try_resolve(self: &Arc<Self>, name: &str) -> Option<SyncSender<()>> {
        self.resolved.begin_insert(&name.to_string())
    }


    pub fn find_const_pool(self: &Arc<Self>, class_name: &str) -> Arc<ConstPool> {
        self.map.get(&class_name.to_string()).unwrap().const_pool.clone()
    }

    pub fn find_method(self: &Arc<Self>, class: &str, method: &str, descriptor: &MethodType) -> Arc<Method> {
        self.map.get(&class.to_string())
            .unwrap().methods.iter()
            .find(|m| m.name.eq(method) && m.descriptor.eq(descriptor))
            .unwrap()
            .clone()
    }
}

#[derive(Clone)]
pub struct Class {
    pub name: String,
    pub super_class: Option<Arc<Class>>,
    pub const_pool: Arc<ConstPool>,
    pub fields: Vec<Arc<Field>>,
    pub methods: Vec<Arc<Method>>,
}

impl Class {
    /// Iterate through the hierarchy of this class, starting at the highest root parent.
    pub fn inverse_hierarchy(self: &Arc<Self>) -> Vec<Arc<Class>> {
        let mut classes = Vec::new();
        let mut class = Some(self.clone());

        while let Some(curr_class) = &class {
            classes.push(curr_class.clone());
            class = curr_class.super_class.clone();
        }

        classes.reverse();

        classes
    }

    pub fn find_instance_method(self: &Arc<Self>, method: &Arc<const_pool::Method>) -> Arc<Method> {
        let mut class = Some(self.clone());
        while let Some(curr_class) = &class {
            let method = curr_class.methods.iter().find(|m|
                !m.is_static && m.name.eq(method.name.as_str()) && m.descriptor.eq(&method.descriptor));

            if let Some(method) = method {
                return method.clone();
            }

            class = curr_class.super_class.clone();
        }
        panic!("Could not find method {:?}", method)
    }

    /// On an object in the heap, the fields are laid out in order from super parent to child,
    /// so we need to find the index offset of a field for the object in the heap.
    pub fn find_field_idx(self: &Arc<Self>, field: &Arc<const_pool::Field>) -> usize {
        let parents = self.inverse_hierarchy();
        let mut idx = 0;

        for parent in &parents {
            for f in &parent.fields {
                if !f.is_static && f.name.eq(field.name.as_str()) && f.descriptor.eq(&field.descriptor) {
                    return idx;
                }
                idx += 1;
            }
        }

        panic!("could not find field {:?} on class {:?}", field, self.name.as_str())
    }
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub is_static: bool,
    pub name: String,
    pub descriptor: FieldType,
}

#[derive(Debug, PartialEq)]
pub struct Method {
    pub is_static: bool,
    pub is_native: bool,
    pub name: String,
    pub descriptor: MethodType,
    pub code: Option<Code>,
}

#[cfg(test)]
mod tests {
    use crate::runtime::Const;

    use super::*;

    // #[test]
    // fn empty_main() {
    //     let runtime = Runtime::new();
    //     let method_area = Arc::new(MethodArea { map: AppendMap::new(), initialized: AppendMap::new() });
    //
    //     let class = method_area.insert(runtime, "EmptyMain");
    //
    //     assert_eq!(class.const_pool.as_ref().len(), 3);
    //     assert_eq!(class.const_pool.get_method(1), Arc::new(crate::runtime::const_pool::Method {
    //         name: "<init>".to_string(),
    //         descriptor: MethodType::from_descriptor("()V").unwrap(),
    //         class: Arc::new(crate::runtime::const_pool::Class { name: "java.lang.Object".to_string() }),
    //     }));
    //     assert_eq!(class.const_pool.get_const(2), &Const::Class(Arc::new(crate::runtime::const_pool::Class { name: "java.lang.Object".to_string() })));
    //     assert_eq!(class.const_pool.get_const(7), &Const::Class(Arc::new(crate::runtime::const_pool::Class { name: "EmptyMain".to_string() })));
    //
    //     assert_eq!(class.methods.len(), 2);
    //     assert_eq!(class.methods.get(0).unwrap(), &Arc::new(Method {
    //         is_native: false,
    //         is_static: false,
    //         name: "<init>".to_string(),
    //         descriptor: MethodType::from_descriptor("()V").unwrap(),
    //         code: Some(Code {
    //             max_stack: 1,
    //             max_locals: 1,
    //             code: vec![0x2a, 0xb7, 0, 1, 0xb1],
    //         }),
    //     }));
    //     assert_eq!(class.methods.get(1).unwrap(), &Arc::new(Method {
    //         is_static: true,
    //         is_native: false,
    //         name: "main".to_string(),
    //         descriptor: MethodType::from_descriptor("([Ljava/lang/String;)V").unwrap(),
    //         code: Some(Code {
    //             max_stack: 0,
    //             max_locals: 1,
    //             code: vec![0xb1],
    //         }),
    //     }))
    // }
}
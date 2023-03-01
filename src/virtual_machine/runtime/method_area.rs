use std::path::Path;
use std::sync::Arc;
use std::sync::mpsc::SyncSender;

use crate::class_file::{ACCESS_FLAG_NATIVE, ACCESS_FLAG_STATIC, Code};
use crate::class_file::loader::Loader;
use crate::collection::AppendMap;
use crate::java::{FieldType, MethodType};
use crate::virtual_machine::runtime::const_pool::ConstPool;
use crate::virtual_machine::runtime::heap::Heap;

pub struct MethodArea {
    map: Arc<AppendMap<String, Class>>,
    /// TODO: Potentially could do with an append set?
    initialized: Arc<AppendMap<String, ()>>,
}

impl MethodArea {
    pub fn new() -> Arc<Self> {
        Arc::new(MethodArea {
            map: AppendMap::new(),
            initialized: AppendMap::new(),
        })
    }

    pub fn insert(self: &Arc<Self>, heap: Arc<Heap>, name: &str) -> Arc<Class> {
        self.map.clone().get_or_insert(&name.to_string(), || {
            let p = Path::new("./classes")
                .join(name.to_string())
                .with_extension("class");
            let mut loader = Loader::new(&p).unwrap();
            let class_file = loader.read_class_file().unwrap();
            let pool = ConstPool::new(&class_file, heap);

            let fields: Vec<Arc<Field>> = class_file.fields.into_iter()
                .map(|f| {
                    let is_static = (ACCESS_FLAG_STATIC & f.access_flags) != 0;
                    let name = if let crate::class_file::Const::Utf8 { bytes } = class_file.const_pool.get(&f.name).unwrap() {
                        String::from_utf8(bytes.clone()).unwrap()
                    } else {
                        panic!()
                    };

                    let descriptor = if let crate::class_file::Const::Utf8 { bytes } = class_file.const_pool.get(&f.descriptor).unwrap() {
                        FieldType::from_descriptor(&String::from_utf8(bytes.clone()).unwrap()).unwrap()
                    } else {
                        panic!()
                    };

                    Arc::new(Field { is_static, name, descriptor })
                })
                .collect();

            let mut is_initialised = true;

            let methods: Vec<Arc<Method>> = class_file.methods.into_iter()
                .map(|m| {
                    let is_static = (m.access_flags & ACCESS_FLAG_STATIC) != 0;
                    let is_native = (m.access_flags & ACCESS_FLAG_NATIVE) != 0;

                    let name = if let crate::class_file::Const::Utf8 { bytes } = class_file.const_pool.get(&m.name).unwrap() {
                        String::from_utf8(bytes.clone()).unwrap()
                    } else {
                        panic!()
                    };

                    if name.eq("<clinit>") {
                        is_initialised = false;
                    }

                    let descriptor = if let crate::class_file::Const::Utf8 { bytes } = class_file.const_pool.get(&m.descriptor).unwrap() {
                        MethodType::from_descriptor(&String::from_utf8(bytes.clone()).unwrap()).unwrap()
                    } else {
                        panic!()
                    };

                    Arc::new(Method { is_static, is_native, name, descriptor, code: m.code })
                }).collect();

            if is_initialised {
                self.initialized.get_or_insert(&name.to_string(), || ());
            }

            Class {
                const_pool: Arc::new(pool),
                fields,
                methods,
            }
        })
    }

    pub fn try_start_initialize(self: &Arc<Self>, class_name: &str) -> Option<SyncSender<()>> {
        self.initialized.begin_insert(&class_name.to_string())
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
    pub const_pool: Arc<ConstPool>,
    pub fields: Vec<Arc<Field>>,
    pub methods: Vec<Arc<Method>>,
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
    use super::*;

    #[test]
    fn empty_main() {
        let heap = Heap::new();
        let method_area = Arc::new(MethodArea { map: AppendMap::new(), initialized: AppendMap::new() });

        let class = method_area.insert(heap, "EmptyMain");

        assert_eq!(class.const_pool.len(), 3);
        assert_eq!(class.const_pool.get_method(1), Arc::new(crate::virtual_machine::runtime::const_pool::Method {
            name: "<init>".to_string(),
            descriptor: MethodType::from_descriptor("()V").unwrap(),
            class: Arc::new(crate::virtual_machine::runtime::const_pool::Class { name: "java.lang.Object".to_string() }),
        }));
        assert_eq!(class.const_pool.get_class(2), Arc::new(crate::virtual_machine::runtime::const_pool::Class { name: "java.lang.Object".to_string() }));
        assert_eq!(class.const_pool.get_class(7), Arc::new(crate::virtual_machine::runtime::const_pool::Class { name: "EmptyMain".to_string() }));

        assert_eq!(class.methods.len(), 2);
        assert_eq!(class.methods.get(0).unwrap(), &Arc::new(Method {
            is_native: false,
            is_static: false,
            name: "<init>".to_string(),
            descriptor: MethodType::from_descriptor("()V").unwrap(),
            code: Some(Code {
                max_stack: 1,
                max_locals: 1,
                code: vec![0x2a, 0xb7, 0, 1, 0xb1],
            }),
        }));
        assert_eq!(class.methods.get(1).unwrap(), &Arc::new(Method {
            is_static: true,
            is_native: false,
            name: "main".to_string(),
            descriptor: MethodType::from_descriptor("([Ljava/lang/String;)V").unwrap(),
            code: Some(Code {
                max_stack: 0,
                max_locals: 1,
                code: vec![0xb1],
            }),
        }))
    }
}
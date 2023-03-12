use maplit::hashmap;
use crate::class_file::Code;
use crate::collection::once::Once;
use crate::java::MethodType;
use crate::method_area::{Class, ClassFlags, Method};
use crate::method_area::const_pool::{ClassKey, Const, ConstPool, MethodKey, SymbolicReference};

pub fn create_main_thread() -> Class {
    Class {
        name: "<create-main-thread>".to_string(),
        flags: ClassFlags { bits: 0 },
        const_pool: ConstPool {
            pool: hashmap! {
                1 => Const::String(SymbolicReference {
                    const_key: "main".to_string(),
                    resolved: Once::new(),
                }),
                2 => Const::Class(SymbolicReference {
                    const_key: ClassKey { name: "java.lang.Thread".to_string() },
                    resolved: Once::new(),
                }),
                3 => Const::Method(SymbolicReference {
                    const_key: MethodKey {
                        class: "java.lang.Thread".to_string(),
                        name: "<init>".to_string(),
                        descriptor: MethodType::from_descriptor("(Ljava/lang/String;)V").unwrap()
                    },
                    resolved: Once::new(),
                })
            }
        },
        super_class: None,
        interfaces: vec![],
        instance_fields: vec![],
        static_fields: vec![],
        methods: vec![
            Method {
                class: 0 as *const Class,
                is_static: true,
                is_native: false,
                is_synchronized: false,
                name: "<create-main-thread>".to_string(),
                descriptor: MethodType::from_descriptor("()Ljava/lang/Thread;").unwrap(),
                code: Some(Code {
                    max_stack: 0,
                    max_locals: 0,
                    code: vec![
                        0xBB, 0, 2, // new java.lang.Thread
                        0x59, // dup thread ref
                        0x12, 1, // ldc string name
                        0xb7, 0, 3, // invokespecial java.lang.Thread.<init>
                        0xb0, // return thread ref
                    ],
                    ex_table: vec![],
                    attributes: vec![],
                }),
            }
        ],
        attributes: vec![],
        instance_width: 0,
        static_width: 0,
        source_file: None,
    }
}
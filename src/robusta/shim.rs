use maplit::hashmap;
use crate::class_file::Code;
use crate::collection::once::Once;
use crate::java::MethodType;
use crate::method_area::{ObjectClass, ClassFlags, Method};
use crate::method_area::const_pool::{ClassKey, Const, ConstPool, MethodKey, SymbolicReference};

pub fn create_main_thread() -> ObjectClass {
    ObjectClass {
        name: "<main-thread>".to_string(),
        flags: ClassFlags { bits: 0 },
        const_pool: ConstPool {
            pool: hashmap! {
                // 1 => Const::Class(SymbolicReference {
                //     const_key: ClassKey { name: "java.lang.ThreadGroup".to_string() },
                //     resolved: Once::new(),
                // }),
                // 2 => Const::Class(SymbolicReference {
                //     const_key: ClassKey { name: "java.lang.Thread".to_string() },
                //     resolved: Once::new(),
                // }),
                // 3 => Const::Method(SymbolicReference {
                //     const_key: MethodKey {
                //         class: "java.lang.ThreadGroup".to_string(),
                //         name: "<init>".to_string(),
                //         descriptor: MethodType::from_descriptor("()V").unwrap(),
                //     },
                //     resolved: Once::new()
                // }),
                //  4 => Const::Method(SymbolicReference {
                //     const_key: MethodKey {
                //         class: "java.lang.ThreadGroup".to_string(),
                //         name: "<init>".to_string(),
                //         descriptor: MethodType::from_descriptor("(Ljava/lang/Void;Ljava/lang/ThreadGroup;Ljava/lang/String;)V").unwrap(),
                //     },
                //     resolved: Once::new()
                // }),
                // 5 => Const::String(SymbolicReference {
                //     const_key: "main".to_string(),
                //     resolved: Once::new()
                // }),
                // 6 => Const::Method(SymbolicReference {
                //     const_key: MethodKey {
                //         class: "java.lang.Thread".to_string(),
                //         name: "<init>".to_string(),
                //         descriptor: MethodType::from_descriptor("(Ljava/lang/ThreadGroup;Ljava/lang/String;)V").unwrap(),
                //     },
                //     resolved: Once::new()
                // }),new
                1 => Const::Class(SymbolicReference {
                    const_key: ClassKey { name: "sun.misc.Launcher".to_string() },
                    resolved: Once::new(),
                }),
                2 => Const::Method(SymbolicReference {
                    const_key: MethodKey {
                        class: "sun.misc.Launcher".to_string(),
                        name: "<init>".to_string(),
                        descriptor: MethodType::from_descriptor("()V").unwrap(),
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
                class: 0 as *const ObjectClass,
                is_static: true,
                is_native: false,
                is_synchronized: false,
                name: "<main-thread>".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
                code: Some(Code {
                    max_stack: 0,
                    max_locals: 0,
                    code: vec![
                        0xBB, 0, 1, // new Launcher
                        0x59,
                        0xB7, 0, 2

                        // 0xBB, 0, 1,     // new ThreadGroup
                        // 0x59,           // dup ThreadGroup ref
                        // 0xB7, 0, 3,     // invokespecial ThreadGroup.<init>()
                        // 0x4B,           // locals[0] = system ThreadGroup
                        //
                        // 0xBB, 0, 1,     // new ThreadGroup
                        // 0x59,           // dup ThreadGroup ref
                        // 0x01,           // push null
                        // 0x2A,           // push locals[0]
                        // 0x12, 5,        // load "main"
                        // 0xB7, 0, 4,     // invokespecial ThreadGroup.<init>(Void,ThreadGroup,String)V
                        // 0x4B,           // locals[0] = main ThreadGroup
                        //
                        // 0x12, 5,        // load "main"
                        // 0x4C,           // locals[1] = "main"
                        //
                        // 0xBB, 0, 2,     // new Thread
                        // 0x59,           // dup Thread ref
                        // 0x2A,           // push locals[0]
                        // 0x2B,           // push locals[1]
                        // 0xB7, 0, 6,     // invokespecial Thread.<init>(ThreadGroup,String)V
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
use std::collections::HashMap;
use std::sync::Arc;
use rand::{RngCore, thread_rng};
use crate::class_file::Code;
use crate::collection::once::Once;

use crate::java::{CategoryOne, FieldType, MethodType, Value};
use crate::method_area;
use crate::method_area::{Class, ClassFlags};
use crate::method_area::const_pool::{ClassKey, Const, ConstPool, FieldKey, MethodKey, SymbolicReference};
use crate::native::{Args, Plugin};
use crate::native::stateless::{Method, stateless};
use crate::runtime::Runtime;

pub fn java_lang_plugins() -> Vec<Box<dyn Plugin>> {
    vec![
        stateless(
            Method {
                class: "java.lang.String".to_string(),
                name: "intern".to_string(),
                descriptor: MethodType::from_descriptor("()Ljava/lang/String;").unwrap(),
            },
            Arc::new(string_intern),
        ),
        stateless(
            Method {
                class: "java.lang.Object".to_string(),
                name: "getClass".to_string(),
                descriptor: MethodType::from_descriptor("()Ljava/lang/Class;").unwrap(),
            },
            Arc::new(object_get_class),
        ),
        stateless(
            Method {
                class: "java.lang.Object".to_string(),
                name: "hashCode".to_string(),
                descriptor: MethodType::from_descriptor("()I").unwrap(),
            },
            Arc::new(object_hash_code),
        ),
        stateless(
            Method {
                class: "java.lang.Throwable".to_string(),
                name: "fillInStackTrace".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(fill_in_stack_trace),
        ),
    ]
}


fn string_intern(args: &Args) -> Option<Value> {
    let string_ref = args.params[0].reference();
    let string_obj = args.runtime.heap.get_object(string_ref);

    let chars_ref = string_obj.get_field(&FieldKey {
        class: "java.lang.String".to_string(),
        name: "chars".to_string(),
        descriptor: FieldType::from_descriptor("[C").unwrap(),
    }).reference();

    let chars = args.runtime.heap.get_array(chars_ref);
    let chars = chars.as_chars_slice();

    let string = String::from_utf16(chars).unwrap();
    let string_ref = args.runtime.heap.insert_string_const(&string, string_obj.class());

    Some(Value::Reference(string_ref))
}

fn object_get_class(args: &Args) -> Option<Value> {
    let object_ref = args.params[0].reference();
    let object_obj = args.runtime.heap.get_object(object_ref);

    let class_ref = args.runtime.method_area.load_class_object(object_obj.class());

    Some(Value::Reference(class_ref))
}

fn object_hash_code(args: &Args) -> Option<Value> {
    let object_ref = args.params[0].reference();
    let object_obj = args.runtime.heap.get_object(object_ref);

    let hash_code = object_obj.hash_code();

    Some(Value::Int(hash_code))
}

fn fill_in_stack_trace(args: &Args) -> Option<Value> {
    let throwable_ref = args.params[0].reference();

    let thread = unsafe { args.thread.cast_mut().as_mut().unwrap() };
    let elems: Vec<StackElem> = thread.stack.iter()
        .map(|frame| {
            let method = unsafe { frame.method.as_ref().unwrap() };
            let class = unsafe { method.class.as_ref().unwrap() };
            StackElem {
                class: frame.class.clone(),
                method: method.name.clone(),
                file: class.source_file.clone(),
                line: {
                    let line_numbers = method.code.as_ref().and_then(|code| code.line_number_table())
                        .map(|table| &table.table);
                    if let Some(table) = line_numbers {
                        table.iter()
                            .filter(|ln| ln.start_pc as usize <= frame.pc)
                            .last()
                            .map(|ln| ln.line_number)
                            .unwrap() as i32
                    } else {
                        -2
                    }
                }
            }
        }).collect();

    // Can we create a class that delegates to all our methods for us?
    let mut class = Class {
        name: format!("<fill-in-stack-trace-{:?}-{}>", std::thread::current().id(), thread_rng().next_u64()),
        flags: ClassFlags { bits: 0 },
        const_pool: ConstPool {
            pool: HashMap::new(),
        },
        super_class: None,
        interfaces: vec![],
        instance_fields: vec![],
        static_fields: vec![],
        methods: vec![],
        attributes: vec![],
        instance_width: 0,
        static_width: 0,
        source_file: None,
    };

    let mut method = method_area::Method {
        class: 0 as *const Class, // TODO: Fill in later!
        is_static: true,
        is_native: false,
        name: "<fill-in-stack-trace>".to_string(),
        descriptor: MethodType::from_descriptor("()[Ljava/lang/StackTraceElement;").unwrap(),
        code: Some(Code {
            max_stack: 0,
            max_locals: 0,
            code: vec![],
            ex_table: vec![],
            attributes: vec![],
        }),
    };

    let code = &mut method.code.as_mut().unwrap().code;

    let mut idx = 1;

    // Class Const
    class.const_pool.pool.insert(idx, Const::Class(SymbolicReference {
        const_key: ClassKey { name: "java.lang.StackTraceElement".to_string() },
        resolved: Once::new(),
    }));
    let class_const_idx = idx;
    idx += 1;

    // Constructor const
    class.const_pool.pool.insert(idx, Const::Method(SymbolicReference {
        const_key: MethodKey {
            class: "java.lang.StackTraceElement".to_string(),
            name: "<init>".to_string(),
            descriptor: MethodType::from_descriptor("(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;I)V").unwrap(),
        },
        resolved: Once::new(),
    }));
    let method_const_idx = idx;
    idx += 1;

    // Length of array const
    class.const_pool.pool.insert(idx, Const::Integer(elems.len() as i32));
    let array_len_const = idx;
    idx += 1;

    // Insert array length constant onto stack
    code.push(0x13); // ldc_w
    code.push(array_len_const.to_be_bytes()[0]);
    code.push(array_len_const.to_be_bytes()[1]);

    // Create new array
    code.push(0xBD); // anewarray
    code.push(class_const_idx.to_be_bytes()[0]);
    code.push(class_const_idx.to_be_bytes()[1]);

    // Store array into local var 0
    code.push(0x4b); // astore_0 -> array into local var 0

    for elem in &elems {
        // Create new element
        code.push(0xBB); // NEW
        code.push(class_const_idx.to_be_bytes()[0]);
        code.push(class_const_idx.to_be_bytes()[1]);

        // Duplicate object ref for later.
        code.push(0x59);

        // Class const & load const.
        class.const_pool.pool.insert(idx, Const::String(SymbolicReference {
            const_key: elem.class.clone(),
            resolved: Once::new(),
        }));
        code.push(0x13); // ldc_w
        code.push(idx.to_be_bytes()[0]);
        code.push(idx.to_be_bytes()[1]);
        idx += 1;

        // Method const & load const
        class.const_pool.pool.insert(idx, Const::String(SymbolicReference {
            const_key: elem.method.clone(),
            resolved: Once::new(),
        }));
        code.push(0x13); // ldc_w
        code.push(idx.to_be_bytes()[0]);
        code.push(idx.to_be_bytes()[1]);
        idx += 1;

        // File const
        if let Some(file) = &elem.file {
            class.const_pool.pool.insert(idx, Const::String(SymbolicReference {
                const_key: file.clone(),
                resolved: Once::new(),
            }));
            code.push(0x13); // ldc_w
            code.push(idx.to_be_bytes()[0]);
            code.push(idx.to_be_bytes()[1]);
            idx += 1;
        } else {
            code.push(0x1); // aconst_null
        }

        // Line Number const & load
        class.const_pool.pool.insert(idx, Const::Integer(elem.line));
        code.push(0x13); // ldc_w
        code.push(idx.to_be_bytes()[0]);
        code.push(idx.to_be_bytes()[1]);
        idx += 1;

        // Invoke constructor
        code.push(0xB7); // invokespecial
        code.push(method_const_idx.to_be_bytes()[0]);
        code.push(method_const_idx.to_be_bytes()[1]);
    }

    // Store index in local_var 1
    code.push(0x3); // 0 const
    code.push(0x3C); // istore_1

    for idx in 0..elems.len() {
        // Pop element off stack into local var 2
        code.push(0x4d);

        // Load array ref from local var 0
        code.push(0x2a);

        // Load index from local var 1
        code.push(0x1b);

        // Load element from local var 2
        code.push(0x2c);

        // Store in array
        code.push(0x53);

        // Increment index in local var 1 by 1
        code.push(0x84);
        code.push(1);
        code.push(1);
    }

    // Load array ref from local var 0
    code.push(0x2a);

    // Return array ref
    code.push(0xb0);

    class.methods.push(method);

    let class = args.runtime.method_area.insert_gen_class(class);
    let method = &unsafe { class.as_ref().unwrap() }.methods[0] as *const method_area::Method;

    let array_reference = thread.native_invoke(class, method).unwrap().reference();

    // Store array reference in field
    let throwable = args.runtime.heap.get_object(throwable_ref);
    throwable.set_field(&FieldKey {
        class: "java.lang.Throwable".to_string(),
        name: "stackTrace".to_string(),
        descriptor: FieldType::from_descriptor("[Ljava.lang.StackTraceElement;").unwrap()
    }, CategoryOne { reference: array_reference });

    None
}

struct StackElem {
    class: String,
    method: String,
    file: Option<String>,
    line: i32,
}
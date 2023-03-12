use std::collections::HashMap;
use std::sync::Arc;
use std::thread::{Builder, sleep};
use std::time::Duration;

use rand::{RngCore, thread_rng};

use crate::method_area;
use crate::class_file::Code;
use crate::collection::once::Once;
use crate::collection::wait::ThreadWait;
use crate::java::{CategoryOne, FieldType, MethodType, Value};
use crate::method_area::{Class, ClassFlags};
use crate::method_area::const_pool::{ClassKey, Const, ConstPool, FieldKey, MethodKey, SymbolicReference};
use crate::native::{Args, Plugin};
use crate::native::stateless::{Method, stateless};
use crate::thread::Thread;

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
        stateless(
            Method {
                class: "java.lang.Integer".to_string(),
                name: "toString".to_string(),
                descriptor: MethodType::from_descriptor("(I)Ljava/lang/String;").unwrap(),
            },
            Arc::new(integer_to_string),
        ),
        stateless(
            Method {
                class: "java.lang.Thread".to_string(),
                name: "nativeStart".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(thread_start),
        ),
        stateless(
            Method {
                class: "java.lang.Thread".to_string(),
                name: "sleep".to_string(),
                descriptor: MethodType::from_descriptor("(J)V").unwrap(),
            },
            Arc::new(thread_sleep),
        ),
        stateless(
            Method {
                class: "java.lang.Thread".to_string(),
                name: "join".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(thread_join),
        ),
        stateless(
            Method {
                class: "java.lang.Thread".to_string(),
                name: "join".to_string(),
                descriptor: MethodType::from_descriptor("(J)V").unwrap(),
            },
            Arc::new(thread_join_millis),
        ),
        stateless(
            Method {
                class: "java.lang.Thread".to_string(),
                name: "currentThread".to_string(),
                descriptor: MethodType::from_descriptor("()Ljava/lang/Thread;").unwrap(),
            },
            Arc::new(current_thread),
        ),
    ]
}

fn integer_to_string(args: &Args) -> Option<Value> {
    let int = args.params[0].int();

    let string_rep = format!("{}", int.0);
    let string_ref = args.runtime.heap.insert_string_const(&string_rep, args.runtime.method_area.load_class("java.lang.String"));

    Some(Value::Reference(string_ref))
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
    let throwable_class = args.runtime.method_area.load_class("java.lang.Throwable");

    let throwable_ref = args.params[0].reference();

    let thread = unsafe { args.thread.cast_mut().as_mut().unwrap() };

    let stack = thread.stack.iter()
        .take_while(|f| {
            let method = unsafe { f.method.as_ref().unwrap() };
            let class = unsafe { method.class.as_ref().unwrap() };
            !class.is_instance_of(throwable_class) && method.name.eq("<init>")
        });

    let elems: Vec<StackElem> = stack.map(|frame| {
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
            },
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

    for _ in 0..elems.len() {
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
        descriptor: FieldType::from_descriptor("[Ljava.lang.StackTraceElement;").unwrap(),
    }, CategoryOne { reference: array_reference });

    None
}

struct StackElem {
    class: String,
    method: String,
    file: Option<String>,
    line: i32,
}

fn thread_start(args: &Args) -> Option<Value> {
    let thread_ref = args.params[0].reference();
    let thread_obj = args.runtime.heap.get_object(thread_ref);

    let name_ref = thread_obj.get_field(&FieldKey {
        class: "java.lang.Thread".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let name = args.runtime.heap.get_string(name_ref);

    let runtime = args.runtime.clone();
    let class = thread_obj.class().name.clone();

    runtime.threads.insert(class.clone(), ThreadWait::new(runtime.clone(), thread_ref));

    Builder::new().name(name.clone()).spawn(move || {
        let const_pool = &thread_obj.class().const_pool as *const ConstPool;
        let method = thread_obj.class().find_method(&MethodKey {
            class: class.clone(),
            name: "run".to_string(),
            descriptor: MethodType::from_descriptor("()V").unwrap(),
        }).unwrap() as *const method_area::Method;

        let thread = Thread::new(name, Some(thread_ref.clone()), runtime, class, const_pool, method);

        // insert local vars!
        thread.as_mut().stack.last_mut().unwrap().local_vars.store_value(0, Value::Reference(thread_ref));

        // hack
        unsafe {
            let t = thread.as_ref() as *const Thread;
            let t = t as *mut Thread;
            let t = t.as_mut().unwrap();
            t.run();
        }
    }).unwrap();

    None
}

pub fn thread_sleep(args: &Args) -> Option<Value> {
    let millis = args.params[0].long().0;
    sleep(Duration::from_millis(millis as u64));
    None
}

pub fn thread_join(args: &Args) -> Option<Value> {
    let thread_ref = args.params[0].reference();
    let thread_obj = args.runtime.heap.get_object(thread_ref);

    let name_ref = thread_obj.get_field(&FieldKey {
        class: "java.lang.Thread".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let name = args.runtime.heap.get_string(name_ref);

    args.runtime.threads.get(&name).unwrap().join();

    None
}

pub fn thread_join_millis(args: &Args) -> Option<Value> {
    let thread_ref = args.params[0].reference();
    let thread_obj = args.runtime.heap.get_object(thread_ref);

    let millis = args.params[1].long();

    let name_ref = thread_obj.get_field(&FieldKey {
        class: "java.lang.Thread".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let name = args.runtime.heap.get_string(name_ref);

    args.runtime.threads.get(&name).unwrap().join_millis(millis.0);

    None
}

pub fn current_thread(args: &Args) -> Option<Value> {
    let thread = unsafe { args.thread.as_ref().unwrap() };
    let thread_ref = thread.reference.as_ref().unwrap();
    Some(Value::Reference(*thread_ref))
}
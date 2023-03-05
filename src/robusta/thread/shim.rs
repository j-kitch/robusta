use std::collections::HashMap;
use std::sync::Arc;

use maplit::hashmap;

use crate::class_file::Code;
use crate::java::{MethodType, Value};
use crate::runtime::{const_pool, ConstPool, Method, Runtime};
use crate::runtime::const_pool::Class;
use crate::thread::Thread;

pub fn intern_string(runtime: Arc<Runtime>, string: &str) -> Thread {
    // println!("{:?} starting intern_string shim", std::thread::current().id());

    let chars: Vec<u16> = string.encode_utf16().collect();
    let arr_ref = runtime.heap.insert_char_arr(&chars);

    let (robusta_class, _) = runtime.method_area.insert(runtime.clone(), "com.jkitch.robusta.Robusta");
    let method = robusta_class.methods.iter().find(|m| m.is_static && m.name.eq("internString")).unwrap();

    let mut thread = Thread::empty(runtime.clone());
    thread.add_frame("<robusta>".into(), Arc::new(ConstPool { pool: HashMap::new() }), Arc::new(Method {
        is_static: true,
        is_native: false,
        name: "<exit>".to_string(),
        descriptor: MethodType::from_descriptor("()V").unwrap(),
        code: Some(Code {
            max_stack: 0,
            max_locals: 0,
            code: vec![0xB1],
        }),
    }));
    thread.add_frame("Robusta".to_string(), robusta_class.const_pool.clone(), method.clone());

    let frame = thread.stack.last_mut().unwrap();
    frame.local_vars.store_value(0, Value::Reference(arr_ref));

    thread
}

pub fn intern_class(runtime: Arc<Runtime>, string: &str) -> Thread {
    // println!("{:?} starting intern_class shim", std::thread::current().id());
    let chars: Vec<u16> = string.encode_utf16().collect();
    let arr_ref = runtime.heap.insert_char_arr(&chars);

    let mut thread = Thread::empty(runtime.clone());
    thread.add_frame("<robusta>".into(), Arc::new(ConstPool { pool: HashMap::new() }), Arc::new(Method {
        is_static: true,
        is_native: false,
        name: "<exit>".to_string(),
        descriptor: MethodType::from_descriptor("()V").unwrap(),
        code: Some(Code {
            max_stack: 0,
            max_locals: 0,
            code: vec![0xB1],
        }),
    }));
    thread.add_frame(
        "<robusta>".to_string(),
        Arc::new(ConstPool { pool: hashmap! {
            1 => const_pool::Const::Method(Arc::new(const_pool::Method {
                class: Arc::new(Class { name: "com.jkitch.robusta.Robusta".to_string() }),
                name: "loadClass".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/String;)Ljava/lang/Class;").unwrap()
            }))
        } }),
        Arc::new(Method {
            is_static: true,
            is_native: false,
            name: "<loadClass>".to_string(),
            descriptor: MethodType::from_descriptor("()V").unwrap(),
            code: Some(Code {
                max_stack: 0,
                max_locals: 0,
                code: vec![
                    0xB8, 0, 1, // invokestatic,
                    0xB1 // return
                ],
            }),
        }));
    thread.add_frame(
        "<robusta>".to_string(),
        Arc::new(ConstPool { pool: hashmap! {
            1 => const_pool::Const::Method(Arc::new(const_pool::Method {
                class: Arc::new(Class { name: "com.jkitch.robusta.Robusta".to_string() }),
                name: "internString".to_string(),
                descriptor: MethodType::from_descriptor("([C)Ljava/lang/String;").unwrap()
            }))
        } }),
        Arc::new(Method {
            is_static: true,
            is_native: false,
            name: "<loadString>".to_string(),
            descriptor: MethodType::from_descriptor("()Ljava/lang/String;").unwrap(),
            code: Some(Code {
                max_stack: 0,
                max_locals: 0,
                code: vec![
                    0xB8, 0, 1, // invokestatic
                    0xB0 // areturn
                ],
            }),
        }));

    let frame = thread.stack.last_mut().unwrap();
    frame.operand_stack.push(Value::Reference(arr_ref));

    thread
}
use std::collections::HashMap;
use std::sync::Arc;
use crate::class_file::Code;

use crate::java::{MethodType, Value};
use crate::runtime::{ConstPool, Method, Runtime};
use crate::thread::{Frame, LocalVars, OperandStack, Thread};

pub fn intern_string(runtime: Arc<Runtime>, string: &str) -> Thread {
    let chars: Vec<u16> = string.encode_utf16().collect();
    let arr_ref = runtime.heap.insert_char_arr(&chars);

    let (robusta_class, _) = runtime.method_area.insert(runtime.clone(), "Robusta");
    let method = robusta_class.methods.iter().find(|m| m.is_static && m.name.eq("internString")).unwrap();

    let mut thread = Thread::new(runtime.clone(), "Robusta".to_string(), robusta_class.const_pool.clone(), method.clone());
    thread.stack.insert(0, Frame {
        class: "<robusta>".to_string(),
        const_pool: Arc::new(ConstPool { pool: HashMap::new() }),
        method: Arc::new(Method {
            is_static: true,
            is_native: false,
            name: "<exit>".to_string(),
            descriptor: MethodType::from_descriptor("()V").unwrap(),
            code: Some(Code {
                max_stack: 0,
                max_locals: 0,
                code: vec![0xB1],
            }),
        }),
        operand_stack: OperandStack::new(),
        local_vars: LocalVars::new(),
        pc: 0,
    });
    let frame = thread.stack.last_mut().unwrap();
    frame.local_vars.store_value(0, Value::Reference(arr_ref));

    thread
}

pub fn load_class(runtime: Arc<Runtime>, class_name: &str) -> Thread {
    todo!()
}
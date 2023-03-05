use std::collections::HashMap;
use std::sync::Arc;

use crate::class_file::Code;
use crate::java::{MethodType, Value};
use crate::runtime::{ConstPool, Method, Runtime};
use crate::thread::Thread;

pub fn intern_string(runtime: Arc<Runtime>, string: &str) -> Thread {
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

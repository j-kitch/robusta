use std::sync::Arc;

use crate::java::Value;
use crate::runtime::Runtime;
use crate::thread::Thread;

pub fn intern_string(runtime: Arc<Runtime>, string: &str) -> Thread {
    let chars: Vec<u16> = string.encode_utf16().collect();
    let arr_ref = runtime.heap.insert_char_arr(&chars);

    let (robusta_class, _) = runtime.method_area.insert(runtime.clone(), "Robusta");
    let method = robusta_class.methods.iter().find(|m| m.is_static && m.name.eq("internString")).unwrap();

    let mut thread = Thread::new(runtime.clone(), "Robusta".to_string(), robusta_class.const_pool.clone(), method.clone());
    let frame = thread.stack.last_mut().unwrap();
    frame.local_vars.store_value(0, Value::Reference(arr_ref));

    thread
}
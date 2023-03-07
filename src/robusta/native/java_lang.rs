use std::collections::HashMap;
use std::sync::Arc;
use std::thread::spawn;

use crate::class_file::Code;
use crate::java::{CategoryOne, FieldType, MethodType, Value};
use crate::method_area::const_pool::FieldKey;
use crate::native::{ Plugin};
use crate::native::stateless::{Method, stateless};
use crate::runtime2::Runtime;
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
        )
    ]
}


fn string_intern(runtime: Arc<Runtime>, values: Vec<CategoryOne>) -> Option<Value> {
    let string_ref = values[0].reference();
    let string_obj = runtime.heap.get_object(string_ref);

    let chars_ref = string_obj.get_field(&FieldKey {
        class: "java.lang.String".to_string(),
        name: "chars".to_string(),
        descriptor: FieldType::from_descriptor("[C").unwrap(),
    }).reference();

    let chars = runtime.heap.get_array(chars_ref);
    let chars = chars.as_chars_slice();

    let string = String::from_utf16(chars).unwrap();
    let string_ref = runtime.heap.insert_string_const(&string, string_obj.class());

    Some(Value::Reference(string_ref))
}

fn object_get_class(runtime: Arc<Runtime>, values: Vec<CategoryOne>) -> Option<Value> {
    let object_ref = values[0].reference();
    let object_obj = runtime.heap.get_object(object_ref);

    let class_ref = runtime.method_area.load_class_object(object_obj.class());

    Some(Value::Reference(class_ref))
}

fn object_hash_code(runtime: Arc<Runtime>, values: Vec<CategoryOne>) -> Option<Value> {
    let object_ref = values[0].reference();
    let object_obj = runtime.heap.get_object(object_ref);

    let hash_code = object_obj.hash_code();

    Some(Value::Int(hash_code))
}
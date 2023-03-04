use std::sync::Arc;

use crate::java::{MethodType, Value};
use crate::native::{Method, Plugin};
use crate::native::stateless::stateless;
use crate::runtime::heap::Array;
use crate::runtime::Runtime;

pub fn robusta_plugins() -> Vec<Box<dyn Plugin>> {
    vec![
        stateless(
            Method {
                class: "Robusta".to_string(),
                name: "println".to_string(),
                descriptor: MethodType::from_descriptor("(I)V").unwrap(),
            },
            Arc::new(robusta_println_int)
        ),
        stateless(
            Method {
                class: "Robusta".to_string(),
                name: "println".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/String;)V").unwrap(),
            },
            Arc::new(robusta_println_string)
        )
    ]
}

fn robusta_println_int(_: Arc<Runtime>, values: Vec<Value>) -> Option<Value> {
    let value = values[0];
    let int = match value {
        Value::Int(int) => int,
        _ => panic!("unexpected")
    };

    println!("{}", int.0);

    None
}

fn robusta_println_string(runtime: Arc<Runtime>, values: Vec<Value>) -> Option<Value> {
    let str_ref = match values[0] {
        Value::Reference(reference) => reference,
        _ => panic!("unexpected")
    };
    let str_obj = runtime.heap.load_object(str_ref);

    let chars_ref = match str_obj.fields.get(0).unwrap().get_value() {
        Value::Reference(reference) => reference,
        _ => panic!("unexpected")
    };
    let chars_arr = runtime.heap.load_array(chars_ref);
    let chars_arr = match chars_arr.as_ref() {
        Array::Char(chars) => chars.clone(),
    };

    let string = String::from_utf16(&chars_arr).unwrap();
    println!("{}", string);

    None
}
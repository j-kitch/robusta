use std::sync::Arc;

use crate::java::{CategoryOne, FieldType, MethodType, Value};
use crate::method_area::const_pool::FieldKey;
use crate::native::{Plugin};
use crate::native::stateless::{Method, stateless};
use crate::runtime2::Runtime;

pub fn robusta_plugins() -> Vec<Box<dyn Plugin>> {
    vec![
        stateless(
            Method {
                class: "com.jkitch.robusta.Robusta".to_string(),
                name: "println".to_string(),
                descriptor: MethodType::from_descriptor("(I)V").unwrap(),
            },
            Arc::new(robusta_println_int),
        ),
        stateless(
            Method {
                class: "com.jkitch.robusta.Robusta".to_string(),
                name: "println".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/String;)V").unwrap(),
            },
            Arc::new(robusta_println_string),
        ),
        // stateless(
        //     Method {
        //         class: "com.jkitch.robusta.Robusta".to_string(),
        //         name: "loadClass".to_string(),
        //         descriptor: MethodType::from_descriptor("(Ljava/lang/String;)Ljava/lang/Class;").unwrap(),
        //     },
        //     Arc::new(load_class),
        // ),
    ]
}

fn robusta_println_int(_: Arc<Runtime>, values: Vec<CategoryOne>) -> Option<Value> {
    let value = values[0].int();

    println!("{}", value.0);

    None
}

fn robusta_println_string(runtime: Arc<Runtime>, values: Vec<CategoryOne>) -> Option<Value> {
    let str_ref = values[0].reference();
    let str_obj = runtime.heap.get_object(str_ref);

    let chars_field = str_obj.get_field(&FieldKey {
        class: "java.lang.String".to_string(),
        name: "chars".to_string(),
        descriptor: FieldType::from_descriptor("[C").unwrap(),
    }).reference();

    let chars_arr = runtime.heap.get_array(chars_field);
    let chars_arr = chars_arr.as_chars_slice();

    let string = String::from_utf16(chars_arr).unwrap();
    println!("{}", string);

    None
}
//
// fn load_class(runtime: Arc<Runtime>, values: Vec<Value>) -> Option<Value> {
//     let str_ref = values[0].reference();
//     let str_obj = runtime.heap.load_object(str_ref);
//     let chars_ref = str_obj.get_field(&const_pool::Field {
//         class: Arc::new(Class { name: "java.lang.String".to_string() }),
//         name: "chars".to_string(),
//         descriptor: FieldType::from_descriptor("[C").unwrap(),
//     }).reference();
//     let chars = runtime.heap.load_array(chars_ref);
//
//     let name = String::from_utf16(chars.as_chars_slice()).unwrap();
//
//     intern_class(runtime, &name)
// }
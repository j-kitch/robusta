use std::process::exit;
use std::sync::Arc;
use tracing::debug;

use crate::java::{FieldType, MethodType, Value};
use crate::log;
use crate::method_area::const_pool::FieldKey;
use crate::native::{Args, Plugin};
use crate::native::stateless::{Method, stateless};

pub fn robusta_plugins() -> Vec<Arc<dyn Plugin>> {
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
        stateless(
            Method {
                class: "com.jkitch.robusta.Robusta".to_string(),
                name: "printerr".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/String;)V").unwrap(),
            },
            Arc::new(robusta_printerr_string),
        ),
        stateless(
            Method {
                class: "com.jkitch.robusta.Robusta".to_string(),
                name: "exit".to_string(),
                descriptor: MethodType::from_descriptor("(I)V").unwrap(),
            },
            Arc::new(robusta_exit)
        )
    ]
}

fn robusta_println_int(args: &Args) -> (Option<Value>, Option<Value>) {
    let value = args.params[0].int();

    println!("{}", value.0);

    (None, None)
}

fn robusta_println_string(args: &Args) -> (Option<Value>, Option<Value>) {
    let str_ref = args.params[0].reference();
    let str_obj = args.runtime.heap.get_object(str_ref);

    let chars_field = str_obj.get_field(&FieldKey {
        class: "java.lang.String".to_string(),
        name: "value".to_string(),
        descriptor: FieldType::from_descriptor("[C").unwrap(),
    }).reference();

    let chars_arr = args.runtime.heap.get_array(chars_field);
    let chars_arr = chars_arr.as_chars_slice();

    let string = String::from_utf16(chars_arr).unwrap();
    println!("{}", string);

    (None, None)
}

fn robusta_printerr_string(args: &Args) -> (Option<Value>, Option<Value>) {
    let str_ref = args.params[0].reference();
    let str_obj = args.runtime.heap.get_object(str_ref);

    let chars_field = str_obj.get_field(&FieldKey {
        class: "java.lang.String".to_string(),
        name: "value".to_string(),
        descriptor: FieldType::from_descriptor("[C").unwrap(),
    }).reference();

    let chars_arr = args.runtime.heap.get_array(chars_field);
    let chars_arr = chars_arr.as_chars_slice();

    let string = String::from_utf16(chars_arr).unwrap();
    eprintln!("{}", string);

    (None, None)
}

fn robusta_exit(args: &Args) -> (Option<Value>, Option<Value>) {
    let code = args.params[0].int().0;
    debug!(target: log::JVM, code, "Exiting JVM");
    exit(code)
}

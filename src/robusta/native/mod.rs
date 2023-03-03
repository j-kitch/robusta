use std::sync::Arc;

use crate::java::{MethodType, Value};
use crate::native::plugin::{Args, Method, Plugins};
use crate::runtime::heap::Array;
use crate::runtime::Runtime;

mod hash_code;
pub mod plugin;

pub struct NativeMethods {
    methods: Vec<NativeMethod>,
    plugins: Plugins,
}

impl NativeMethods {
    pub fn new() -> Self {
        NativeMethods {
            methods: vec![
                NativeMethod {
                    class: "Robusta".to_string(),
                    name: "println".to_string(),
                    descriptor: MethodType::from_descriptor("(I)V").unwrap(),
                    function: Arc::new(robusta_println_int),
                },
                NativeMethod {
                    class: "Robusta".to_string(),
                    name: "println".to_string(),
                    descriptor: MethodType::from_descriptor("(Ljava/lang/String;)V").unwrap(),
                    function: Arc::new(robusta_println_string),
                },
            ],
            plugins: Plugins::new(),
        }
    }

    pub fn call(&self, method: &Method, args: &Args) -> Option<Value> {
        let func = self.methods.iter().find(|m| {
            m.class.eq(&method.class) && m.name.eq(&method.name) && m.descriptor.eq(&method.descriptor)
        }).map(|m| m.function.clone());

        if func.is_some() {
            let func = func.unwrap();
            func(args.runtime.clone(), args.params.clone())
        } else {
            self.plugins.call(method, args)
        }
    }
}

pub struct NativeMethod {
    pub class: String,
    pub name: String,
    pub descriptor: MethodType,
    pub function: Function,
}

type Function = Arc<dyn Fn(Arc<Runtime>, Vec<Value>) -> Option<Value> + Sync + Send>;

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

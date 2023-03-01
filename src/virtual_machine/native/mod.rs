use std::sync::Arc;
use crate::java::{MethodType, Value};

pub struct NativeMethods {
    methods: Vec<NativeMethod>
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
                }
            ]
        }
    }

    pub fn find(&self, class: &str, name: &str, descriptor: &MethodType) -> Option<Function> {
        println!("Looking for {} {} {:?}", class, name, descriptor);
        self.methods.iter().find(|m| {
            m.name.eq(name) && m.descriptor.eq(descriptor) && m.class.eq(class)
        }).map(|m| m.function.clone())
    }
}

pub struct NativeMethod {
    pub class: String,
    pub name: String,
    pub descriptor: MethodType,
    pub function: Function,
}

type Function = Arc<dyn Fn(Vec<Value>) -> Option<Value> + Sync + Send>;

fn robusta_println_int(values: Vec<Value>) -> Option<Value> {
    let value = values[0];
    let int = match value {
        Value::Int(int) => int,
        _ => panic!("unexpected")
    };

    println!("{}", int.0);

    None
}

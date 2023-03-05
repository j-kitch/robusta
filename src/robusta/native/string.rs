use std::sync::Arc;
use crate::java::{MethodType, Value};
use crate::native::{Method, Plugin};
use crate::native::stateless::stateless;
use crate::runtime::Runtime;

pub fn string_plugins() -> Vec<Box<dyn Plugin>> {
    vec![
        stateless(
            Method {
                class: "java.lang.String".to_string(),
                name: "intern".to_string(),
                descriptor: MethodType::from_descriptor("()Ljava/lang/String;").unwrap(),
            },
            Arc::new(string_intern),
        ),
    ]
}


fn string_intern(runtime: Arc<Runtime>, values: Vec<Value>) -> Option<Value> {
    let string_ref = values[0].reference();

    let interned_string_ref = runtime.heap.intern_string(string_ref);

    Some(Value::Reference(interned_string_ref))
}

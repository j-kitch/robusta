use std::sync::Arc;
use crate::java::{MethodType, Value};
use crate::method_area;
use crate::method_area::ObjectClass;
use crate::method_area::const_pool::MethodKey;
use crate::native::{Args, Plugin};
use crate::native::stateless::{Method, stateless};

pub fn system_plugins() -> Vec<Arc<dyn Plugin>> {
    vec![
        stateless(
            Method {
                class: "java.lang.System".to_string(),
                name: "registerNatives".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(register_natives),
        ),
        stateless(
            Method {
                class: "java.lang.System".to_string(),
                name: "initProperties".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/util/Properties;)Ljava/util/Properties;").unwrap(),
            },
            Arc::new(init_properties),
        )
    ]
}

fn register_natives(args: &Args) -> (Option<Value>, Option<Value>) {
    let system_class = args.runtime.method_area.load_class("java.lang.System");

    let init_method = system_class.find_method(&MethodKey {
        class: "java.lang.System".to_string(),
        name: "initializeSystemClass".to_string(),
        descriptor: MethodType::from_descriptor("()V").unwrap(),
    }).unwrap();

    let thread = unsafe { args.thread.cast_mut().as_mut().unwrap() };

    let (_, ex) = thread.native_invoke(&*system_class as *const ObjectClass, init_method as *const method_area::Method, vec![]);
    (None, ex)
}

fn init_properties(args: &Args) -> (Option<Value>, Option<Value>) {
    (Some(args.params[0]), None)
}
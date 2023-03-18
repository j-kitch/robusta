use std::sync::Arc;
use crate::java::{MethodType, Value};
use crate::method_area;
use crate::method_area::const_pool::MethodKey;
use crate::native::{Args, Plugin};
use crate::native::stateless::{Method, stateless};

pub fn java_security_plugins() -> Vec<Arc<dyn Plugin>> {
    vec![
        stateless(
            Method {
                class: "java.security.AccessController".to_string(),
                name: "doPrivileged".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava.security.PrivilegedExceptionAction;)Ljava/lang/Object;").unwrap(),
            },
            Arc::new(do_privileged),
        ),
        stateless(
            Method {
                class: "java.security.AccessController".to_string(),
                name: "doPrivileged".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava.security.PrivilegedAction;)Ljava/lang/Object;").unwrap(),
            },
            Arc::new(do_privileged_2),
        )
    ]
}

fn do_privileged(args: &Args) -> Option<Value> {
    let action_ref = args.params[0].reference();
    let action_obj = args.runtime.heap.get_object(action_ref);
    let action_class = unsafe { action_obj.header.as_ref().unwrap().class.as_ref().unwrap() };

    let method = action_class.find_method(&MethodKey {
        class: "java.security.PrivilegedExceptionAction".to_string(),
        name: "run".to_string(),
        descriptor: MethodType::from_descriptor("()Ljava/lang/Object;").unwrap(),
    }).unwrap();

    let thread = unsafe { args.thread.cast_mut().as_mut().unwrap() };
    let result = thread.native_invoke(
        method.class,
        method as *const method_area::Method,
        vec![Value::Reference(action_ref)]
    ).unwrap();

    Some(result)
}

fn do_privileged_2(args: &Args) -> Option<Value> {
    let action_ref = args.params[0].reference();
    let action_obj = args.runtime.heap.get_object(action_ref);
    let action_class = unsafe { action_obj.header.as_ref().unwrap().class.as_ref().unwrap() };

    let method = action_class.find_method(&MethodKey {
        class: "java.security.PrivilegedAction".to_string(),
        name: "run".to_string(),
        descriptor: MethodType::from_descriptor("()Ljava/lang/Object;").unwrap(),
    }).unwrap();

    let thread = unsafe { args.thread.cast_mut().as_mut().unwrap() };
    let result = thread.native_invoke(
        method.class,
        method as *const method_area::Method,
        vec![Value::Reference(action_ref)]
    ).unwrap();

    Some(result)
}



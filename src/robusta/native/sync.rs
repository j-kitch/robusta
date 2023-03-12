use std::sync::Arc;
use crate::java::{MethodType, Value};
use crate::native::{Args, Plugin};
use crate::native::stateless::{Method, stateless};

pub fn sync_plugins() -> Vec<Box<dyn Plugin>> {
    vec![
        stateless(
            Method {
                class: "java.lang.Object".to_string(),
                name: "wait".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(wait),
        ),
        stateless(
            Method {
                class: "java.lang.Object".to_string(),
                name: "notify".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(notify),
        ),
        stateless(
            Method {
                class: "java.lang.Object".to_string(),
                name: "notifyAll".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(notify_all),
        )
    ]
}

fn wait(args: &Args) -> Option<Value> {
    let thread = unsafe { args.thread.as_ref().unwrap() };

    let this_ref = args.params[0].reference();
    let this_obj = thread.runtime.heap.get_object(this_ref);
    let header = unsafe { this_obj.header.as_ref().unwrap() };

    header.lock.wait(thread.reference.expect("required").0, None);

    None
}

fn notify(args: &Args) -> Option<Value> {
    let thread = unsafe { args.thread.as_ref().unwrap() };

    let this_ref = args.params[0].reference();
    let this_obj = thread.runtime.heap.get_object(this_ref);
    let header = unsafe { this_obj.header.as_ref().unwrap() };

    header.lock.notify_one(thread.reference.expect("required").0);

    None
}

fn notify_all(args: &Args) -> Option<Value> {
    let thread = unsafe { args.thread.as_ref().unwrap() };

    let this_ref = args.params[0].reference();
    let this_obj = thread.runtime.heap.get_object(this_ref);
    let header = unsafe { this_obj.header.as_ref().unwrap() };

    header.lock.notify_all(thread.reference.expect("required").0);

    None
}

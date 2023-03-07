use tracing::info;
// use crate::instruction::new::{resolve_class, resolve_method};
use crate::java::{CategoryOne, CategoryTwo, Value};
use crate::method_area::const_pool::{ConstPool, MethodKey};
use crate::method_area::Method;
use crate::native::{Args};
use crate::thread::Thread;

/// No difference between these two methods YET
pub fn invoke_virtual(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let method_idx = cur_frame.read_u16();

    let method = thread.runtime.method_area.resolve_method(cur_frame.const_pool, method_idx);
    let method = unsafe { method.as_ref().unwrap() };

    let mut args: Vec<CategoryOne> = (0..method.descriptor.parameters.len() + 1)
        .map(|_| cur_frame.operand_stack.pop_cat_one())
        .collect();
    args.reverse();

    let object_ref = args[0].reference();
    let object = thread.runtime.heap.get_object(object_ref);

    // resolve_class(thread.runtime.clone(), object.class_name.as_str());
    // let (object_class, _) = thread.runtime.method_area.insert(thread.runtime.clone(), object.class().as_ref().name.as_str());

    // Find method
    // let (class, method) = object_class.find_instance_method(&method);
    // resolve_method(thread.runtime.clone(), &method.clone());

    let method = object.class().find_method(&MethodKey {
        class: object.class().name.clone(),
        name: method.name.clone(),
        descriptor: method.descriptor.clone(),
    });
    let class = unsafe { method.class.as_ref().unwrap() };

    if method.is_native {
        let result = thread.runtime.native.call(
            method,
            &Args {
                params: args,
                runtime: thread.runtime.clone(),
            }
        );


        if let Some(result) = result {
            cur_frame.operand_stack.push_value(result);
        }
    } else {
        thread.push_frame(class.name.clone(), &class.const_pool as *const ConstPool, method as *const Method, args);
    }
}

pub fn invoke_special(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let method_idx = cur_frame.read_u16();

    // TODO: Not Handling interface methods here
    let method = thread.runtime.method_area.resolve_method(cur_frame.const_pool, method_idx);
    let method = unsafe { method.as_ref().unwrap() };
    let class = unsafe { method.class.as_ref().unwrap() };

    let mut args: Vec<CategoryOne> = (0..method.descriptor.parameters.len() + 1)
        .map(|_| cur_frame.operand_stack.pop_cat_one())
        .collect();
    args.reverse();

    if method.is_native {
        let result = thread.runtime.native.call(
            method,
            &Args {
                params: args,
                runtime: thread.runtime.clone(),
            }
        );

        if let Some(result) = result {
            cur_frame.operand_stack.push_value(result);
        }
    } else {
        thread.push_frame(class.name.clone(), &class.const_pool as *const ConstPool, method as *const Method, args);
    }
}
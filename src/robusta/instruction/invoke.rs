use tracing::debug;

use crate::java::CategoryOne;
use crate::log;
use crate::method_area::const_pool::{ConstPool, MethodKey};
use crate::method_area::Method;
use crate::thread::Thread;

/// No difference between these two methods YET
pub fn invoke_virtual(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let method_idx = cur_frame.read_u16();

    let method = thread.runtime.method_area.resolve_method(thread.runtime.clone(), cur_frame.const_pool, method_idx);
    let method = unsafe { method.as_ref().unwrap() };

    let mut args: Vec<CategoryOne> = (0..method.descriptor.parameters.len() + 1)
        .map(|_| cur_frame.operand_stack.pop_cat_one())
        .collect();
    args.reverse();

    let object_ref = args[0].reference();
    let object = thread.runtime.heap.get_object(object_ref);

    let method = object.class().find_method(&MethodKey {
        class: object.class().name.clone(),
        name: method.name.clone(),
        descriptor: method.descriptor.clone(),
    }).unwrap();
    let class = unsafe { method.class.as_ref().unwrap() };

    if method.is_native {
        let result = thread.call_native(
            method,
            args,
        );

        if let Some(result) = result {
            let cur_frame = thread.stack.last_mut().unwrap();
            cur_frame.operand_stack.push_value(result);
        }
    } else {
        debug!(target: log::INSTR, method=format!("{}.{}{}", class.name.as_str(), method.name.as_str(), method.descriptor.descriptor()), "Invoking virtual method");
        thread.push_frame(class.name.clone(), &class.const_pool as *const ConstPool, method as *const Method, args);
    }
}

pub fn invoke_special(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let method_idx = cur_frame.read_u16();

    // TODO: Not Handling interface methods here
    let method = thread.runtime.method_area.resolve_method(thread.runtime.clone(), cur_frame.const_pool, method_idx);
    let method = unsafe { method.as_ref().unwrap() };
    let class = unsafe { method.class.as_ref().unwrap() };

    let mut args: Vec<CategoryOne> = (0..method.descriptor.parameters.len() + 1)
        .map(|_| cur_frame.operand_stack.pop_cat_one())
        .collect();
    args.reverse();

    if method.is_native {
        let result = thread.call_native(
            method,
            args,
        );

        if let Some(result) = result {
            let cur_frame = thread.stack.last_mut().unwrap();
            cur_frame.operand_stack.push_value(result);
        }
    } else {
        debug!(target: log::INSTR, method=format!("{}.{}{}", class.name.as_str(), method.name.as_str(), method.descriptor.descriptor()), "Invoking special method");
        thread.push_frame(class.name.clone(), &class.const_pool as *const ConstPool, method as *const Method, args);
    }
}
use crate::instruction::new::{resolve_class, resolve_method};
use crate::java::Value;
use crate::native::{Args, Method};
use crate::thread::Thread;

/// No difference between these two methods YET
pub fn invoke_virtual(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let method_idx = cur_frame.read_u16();

    // TODO: Not Handling interface methods here
    let method = cur_frame.const_pool.get_method(method_idx);
    // println!("{} invokevirtual - {}.{}{}", thread.group.as_str(), method.class.name.as_str(), method.name.as_str(), method.descriptor.descriptor());
    resolve_class(thread.runtime.clone(), method.class.name.as_str());

    let mut args: Vec<Value> = (0..method.descriptor.parameters.len() + 1)
        .map(|_| cur_frame.operand_stack.pop())
        .collect();
    args.reverse();

    let object_ref = args[0];
    let object = thread.runtime.heap.load_object(object_ref.reference());

    // resolve_class(thread.runtime.clone(), object.class_name.as_str());
    let (object_class, _) = thread.runtime.method_area.insert(thread.runtime.clone(), object.class().as_ref().name.as_str());

    // Find method
    let (class, method) = object_class.find_instance_method(&method);
    resolve_method(thread.runtime.clone(), &method.clone());

    if method.is_native {
        let result = thread.runtime.native.call(
            &Method {
                class: class.name.clone(),
                name: method.name.clone(),
                descriptor: method.descriptor.clone()
            },
            &Args {
                params: args,
                runtime: thread.runtime.clone(),
            }
        );

        if let Some(result) = result {
            cur_frame.operand_stack.push(result);
        }
    } else {
        thread.push_frame(object.class().name.clone(), object_class.const_pool.clone(), method.clone(), args);
    }
}

pub fn invoke_special(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let method_idx = cur_frame.read_u16();

    // TODO: Not Handling interface methods here
    let method = cur_frame.const_pool.get_method(method_idx);
    // println!("{} invokespecial - {}.{}{}", thread.group.as_str(), method.class.name.as_str(), method.name.as_str(), method.descriptor.descriptor());
    resolve_class(thread.runtime.clone(), method.class.name.as_str());

    let (class, _) = thread.runtime.method_area.insert(thread.runtime.clone(), method.class.name.as_str());
    let (class, method) = class.find_instance_method(&method);

    let mut args: Vec<Value> = (0..method.descriptor.parameters.len() + 1)
        .map(|_| cur_frame.operand_stack.pop())
        .collect();
    args.reverse();

    if method.is_native {
        let result = thread.runtime.native.call(
            &Method {
                class: class.name.clone(),
                name: method.name.clone(),
                descriptor: method.descriptor.clone()
            },
            &Args {
                params: args,
                runtime: thread.runtime.clone(),
            }
        );

        if let Some(result) = result {
            cur_frame.operand_stack.push(result);
        }
    } else {
        thread.push_frame(class.name.clone(), class.const_pool.clone(), method.clone(), args);
    }
}
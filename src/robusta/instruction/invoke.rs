use crate::instruction::new::resolve_class;
use crate::java::Value;
use crate::thread::Thread;

/// No difference between these two methods YET
pub fn invoke_virtual(thread: &mut Thread) {
    invoke_special(thread)
}

pub fn invoke_special(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let method_idx = cur_frame.read_u16();

    // TODO: Not Handling interface methods here
    let method = cur_frame.const_pool.get_method(method_idx);
    resolve_class(thread.runtime.clone(), method.class.name.as_str());

    let (class, _) = thread.runtime.method_area.insert(thread.runtime.clone(), method.class.name.as_str());
    let method = class.find_instance_method(&method);

    let args: Vec<Value> = (0..method.descriptor.parameters.len() + 1)
        .map(|_| cur_frame.operand_stack.pop())
        .rev()
        .collect();

    if method.is_native {
        panic!("not implemented yet")
    } else {
        thread.push_frame(class.const_pool.clone(), method.clone(), args);
    }
}
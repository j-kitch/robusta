use tracing::{debug, trace};

use crate::log;
use crate::method_area::Method;
use crate::method_area::const_pool::{ConstPool, MethodKey};
use crate::thread::Thread;

/// No difference between these two methods YET
pub fn invoke_virtual(thread: &mut Thread) {
    invoke(thread, "invokevirtual", false, true)
}

pub fn invoke_special(thread: &mut Thread) {
    invoke(thread, "invokespecial", false, false)
}

/// Instruction `invokestatic` invokes a class static method.
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.invokestatic).
pub fn invoke_static(thread: &mut Thread) {
    invoke(thread, "invokestatic", true, false)
}

fn invoke(thread: &mut Thread, name: &str, is_static: bool, is_virtual: bool) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let method_idx = cur_frame.read_u16();

    trace!(target: log::INSTR, pc=cur_frame.pc - 3, opcode=name, index=method_idx);

    let cur_frame = thread.stack.last_mut().unwrap();
    let method = thread.runtime.method_area.resolve_method(thread.runtime.clone(), cur_frame.const_pool, method_idx);
    let method = unsafe { method.as_ref().unwrap() };

    let cur_frame = thread.stack.last_mut().unwrap();
    let args = cur_frame.pop_args(is_static, &method.descriptor);

    let method = if is_static || !is_virtual {
        method as *const Method
    } else {
        let object_ref = args[0].reference();
        let object = thread.runtime.heap.get_object(object_ref);
        let object_class = object.class();

        object_class.find_method(&MethodKey {
            class: object.class().name.clone(),
            name: method.name.clone(),
            descriptor: method.descriptor.clone(),
        }).unwrap() as *const Method
    };

    let method = unsafe { method.as_ref().unwrap() };
    let class = unsafe { method.class.as_ref().unwrap() };

    if method.is_synchronized {
        let this_ref = if is_static {
            thread.runtime.heap.get_static(class)
        } else {
            args[0].reference()
        };
        thread.enter_monitor(this_ref);
    }

    if method.is_native {
        debug!(target: log::INSTR, method=format!("{}.{}{}", class.name.as_str(), method.name.as_str(), method.descriptor.descriptor()), "Invoking native method");
        let native_method = thread.find_native(method).unwrap();
        thread.push_native(class.name.clone(), &class.const_pool as *const ConstPool, method as *const Method, args, native_method);
    } else {
        debug!(target: log::INSTR, method=format!("{}.{}{}", class.name.as_str(), method.name.as_str(), method.descriptor.descriptor()), "Invoking method");
        thread.push_frame(class.name.clone(), &class.const_pool as *const ConstPool, method as *const Method, args);
    }
}

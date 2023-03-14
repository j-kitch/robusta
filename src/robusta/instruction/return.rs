use crate::java::{ MethodType, Value};
use crate::method_area;
use crate::method_area::const_pool::{ConstPool, MethodKey};
use crate::thread::Thread;

pub fn a_return(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let method = unsafe { cur_frame.method.as_ref().unwrap() };

    let reference = cur_frame.operand_stack.pop();

    if method.is_synchronized {
        let this_ref = if method.is_static {
            thread.runtime.heap.get_static(unsafe { method.class.as_ref().unwrap() })
        } else {
            cur_frame.local_vars.load_cat_one(0).reference()
        };
        thread.exit_monitor(this_ref);
    }

    thread.stack.pop();
    let cur_frame = thread.stack.last_mut().unwrap();

    cur_frame.operand_stack.push_value(reference);
}

pub fn i_return(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let method = unsafe { cur_frame.method.as_ref().unwrap() };

    let int = cur_frame.operand_stack.pop();

    if method.is_synchronized {
        let this_ref = if method.is_static {
            thread.runtime.heap.get_static(unsafe { method.class.as_ref().unwrap() })
        } else {
            cur_frame.local_vars.load_cat_one(0).reference()
        };
        thread.exit_monitor(this_ref);
    }

    thread.stack.pop();
    let cur_frame = thread.stack.last_mut().unwrap();

    cur_frame.operand_stack.push_value(int);
}

pub fn a_throw(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let throwable_ref = frame.operand_stack.pop().reference();
    let throwable = thread.runtime.heap.get_object(throwable_ref);
    let throw_class = throwable.class();

    let mut frame = Some(frame);
    while let Some(current_frame) = frame {
        let method = unsafe { current_frame.method.as_ref().unwrap() };
        let code = method.code.as_ref().unwrap();
        let ex_table = &code.ex_table;
        for handler in ex_table {
            let in_range = (handler.start_pc as usize) < current_frame.pc && current_frame.pc <= handler.end_pc as usize;
            if !in_range {
                continue;
            }
            let is_handler = handler.catch_type == 0 || {
                let pool = current_frame.const_pool;
                thread.safe.enter();
                let catch_class = thread.runtime.method_area.resolve_class(pool, handler.catch_type);
                thread.safe.exit();
                let catch_class = unsafe { catch_class.as_ref().unwrap() };
                throw_class.is_instance_of(catch_class)
            };
            if is_handler {
                current_frame.pc = handler.handler_pc as usize;
                current_frame.operand_stack.push_value(Value::Reference(throwable_ref));
                return;
            }
        }

        if method.is_synchronized {
            let this_ref = if method.is_static {
                thread.runtime.heap.get_static(unsafe { method.class.as_ref().unwrap() })
            } else {
                current_frame.local_vars.load_cat_one(0).reference()
            };
            thread.exit_monitor(this_ref);
        }

        // No handler found
        thread.stack.pop();
        frame = thread.stack.last_mut();
    }

    // Invoke throwable printStackTrace
    let throwable_method = throw_class.find_method(&MethodKey {
        class: "java.lang.Throwable".to_string(),
        name: "stackTraceAndExit".to_string(),
        descriptor: MethodType::from_descriptor("()V").unwrap(),
    }).unwrap();

    let method_class = unsafe { throwable_method.class.as_ref().unwrap() };

    thread.push_frame(
        method_class.name.clone(),
        &method_class.const_pool as *const ConstPool,
        throwable_method as *const method_area::Method,
        vec![Value::Reference(throwable_ref)]);
}
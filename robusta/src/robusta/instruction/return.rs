use std::io::{stderr, Write};
use tracing::debug;
use crate::{log, method_area};
use crate::java::{MethodType, Value};
use crate::method_area::const_pool::{ConstPool, MethodKey};
use crate::thread::Thread;

/// Instruction `return`
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.return).
pub fn r#return(thread: &mut Thread) {
    exit_monitor(thread);

    thread.stack.pop();
}

pub fn a_return(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let reference = cur_frame.operand_stack.pop();

    exit_monitor(thread);

    thread.stack.pop();
    let cur_frame = thread.stack.last_mut().unwrap();

    cur_frame.operand_stack.push(reference);
}

pub fn i_return(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let int = cur_frame.operand_stack.pop();

    exit_monitor(thread);

    thread.stack.pop();
    let cur_frame = thread.stack.last_mut().unwrap();

    cur_frame.operand_stack.push(int);
}

pub fn f_return(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let float = cur_frame.operand_stack.pop();

    exit_monitor(thread);

    thread.stack.pop();
    let cur_frame = thread.stack.last_mut().unwrap();

    cur_frame.operand_stack.push(float);
}

pub fn d_return(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let double = cur_frame.operand_stack.pop();

    exit_monitor(thread);

    thread.stack.pop();
    let cur_frame = thread.stack.last_mut().unwrap();

    cur_frame.operand_stack.push(double);
}

pub fn l_return(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let long = cur_frame.operand_stack.pop();

    exit_monitor(thread);

    thread.stack.pop();
    let cur_frame = thread.stack.last_mut().unwrap();

    cur_frame.operand_stack.push(long);
}

#[allow(unreachable_code)]
pub fn a_throw(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let throwable_ref = frame.operand_stack.pop().reference();
    let throwable = thread.runtime.heap.get_object(throwable_ref);
    let throw_class = throwable.class();

    let mut frame = Some(frame);
    while let Some(current_frame) = frame {
        if current_frame.class.eq("<native-callback>") {
            // We need to stop and let the native method decide how to handle the exception!
            current_frame.native_ex = Some(throwable_ref);
            return;
        }
        if current_frame.class.starts_with('<') {
            thread.stack.pop();
            frame = thread.stack.last_mut();
            continue;
        }
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
                let catch_class = thread.runtime.method_area.resolve_class(pool, handler.catch_type);
                throw_class.is_instance_of(&catch_class.obj())
            };
            if is_handler {
                current_frame.pc = handler.handler_pc as usize;
                current_frame.operand_stack.push(Value::Reference(throwable_ref));
                return;
            }
        }

        exit_monitor(thread);

        // No handler found
        thread.stack.pop();
        frame = thread.stack.last_mut();
    }

    stderr().write_fmt(format_args!("Exception in thread \"{}\" ", &thread.name)).unwrap();

    // Invoke throwable printStackTrace
    let throwable_method = throw_class.find_method(&MethodKey {
        class: "java.lang.Throwable".to_string(),
        name: "printStackTrace".to_string(),
        descriptor: MethodType::from_descriptor("()V").unwrap(),
    }).unwrap();

    let method_class = unsafe { throwable_method.class.as_ref().unwrap() };

    thread.push_frame(
        method_class.name.clone(),
        &method_class.const_pool as *const ConstPool,
        throwable_method as *const method_area::Method,
        vec![Value::Reference(throwable_ref)]);

    debug!(target: log::THREAD, "Invoking java.lang.Throwable.printStackTrace()V");
}

fn exit_monitor(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let method = unsafe { frame.method.as_ref().unwrap() };
    if method.is_synchronized {
        let monitor_ref = if method.is_static {
            thread.runtime.heap.get_static(unsafe { method.class.as_ref().unwrap() })
        } else {
            frame.local_vars.load_cat_one(0).reference()
        };
        thread.exit_monitor(monitor_ref);
    }
}
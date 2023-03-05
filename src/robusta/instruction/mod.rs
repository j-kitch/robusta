use std::thread::spawn;
pub use new::new;

use crate::instruction::new::{resolve_class, resolve_method};
use crate::java::Value;
use crate::native::{Args, Method};
use crate::runtime::Const;
use crate::thread::shim::{intern_class, intern_string};
use crate::thread::Thread;

pub mod new;
pub mod dup;
pub mod invoke;
pub mod field;
pub mod r#return;
pub mod r#const;
pub mod array;
pub mod math;
pub mod branch;
pub mod locals;
pub mod stack;

/// Instruction `ldc`
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.ldc).
pub fn load_constant(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let code = cur_frame.method.code.as_ref().unwrap();

    let index = code.code[cur_frame.pc] as u16;
    cur_frame.pc += 1;

    match cur_frame.const_pool.get_const(index) {
        Const::String(string) => {

            let mut intern_string_thread = intern_string(thread.runtime.clone(), string.string.as_str());
            let handle = spawn(move || intern_string_thread.run());
            handle.join().unwrap();

            let string_ref = thread.runtime.heap.get_string(string.string.as_str());
            cur_frame.operand_stack.push(Value::Reference(string_ref));
        }
        Const::Integer(int) => {
            cur_frame.operand_stack.push(Value::Int(int.int));
        }
        Const::Class(class) => {
            resolve_class(thread.runtime.clone(), class.name.as_str());

            let mut intern_class_thread = intern_class(thread.runtime.clone(), class.name.as_ref());
            let handle = spawn(move || intern_class_thread.run());
            handle.join().unwrap();
            // println!("Finished interning class");
            let class_ref = thread.runtime.heap.get_class(class.name.as_ref());
            cur_frame.operand_stack.push(Value::Reference(class_ref));
        }
        other => panic!("Not supported const {:?}", other)
    }
}

/// astore_<n>
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.astore_n).
pub fn astore_n(thread: &mut Thread, n: u16) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let value = cur_frame.operand_stack.pop();
    match &value {
        Value::Reference(_) | Value::ReturnAddress(_) => {}
        _ => panic!("Expected a reference or return address")
    }

    cur_frame.local_vars.store_value(n, value);
}

/// istore_<n>
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.istore_n).
pub fn istore_n(thread: &mut Thread, n: u16) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let int = match cur_frame.operand_stack.pop() {
        Value::Int(int) => int,
        _ => panic!("unsupported operation")
    };

    cur_frame.local_vars.store_value(n, Value::Int(int));
}

/// iload_<n>
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.iload_n).
pub fn iload_n(thread: &mut Thread, n: u16) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let int = cur_frame.local_vars.load_int(n);

    cur_frame.operand_stack.push(Value::Int(int));
}

/// aload_<n>
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.aload_n).
pub fn aload_n(thread: &mut Thread, n: u16) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let reference = cur_frame.local_vars.load_ref(n);

    cur_frame.operand_stack.push(Value::Reference(reference));
}

/// Instruction `return`
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.return).
pub fn r#return(thread: &mut Thread) {
    thread.stack.pop();
}

/// Instruction `invokestatic` invokes a class static method.
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.invokestatic).
pub fn invoke_static(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let method_idx = cur_frame.read_u16();
    let method = cur_frame.const_pool.get_method(method_idx);

    // Load the class if not loaded.
    let (class, _) = thread.runtime.method_area.insert(thread.runtime.clone(), method.class.name.as_str());
    resolve_class(thread.runtime.clone(), class.name.as_str());

    // We can call the static method now
    let method = thread.runtime.method_area.find_method(method.class.name.as_str(), method.name.as_str(), &method.descriptor);
    resolve_method(thread.runtime.clone(), &method);

    if !method.is_static {
        panic!("Expected a static method");
    }

    let args: Vec<Value> = (0..method.descriptor.parameters.len())
        .map(|_| cur_frame.operand_stack.pop())
        .rev()
        .collect();

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
        thread.add_frame(class.name.clone(), class.const_pool.clone(), method);
        let frame = thread.stack.last_mut().unwrap();
        let mut idx = 0;
        for param in &args {
            frame.local_vars.store_value(idx, param.clone());
            idx += param.category() as u16;
        }
    }
}
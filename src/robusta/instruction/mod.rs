use std::thread::spawn;
use crate::java::{MethodType, Value};
use crate::runtime::Const;
use crate::thread::Thread;

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
            cur_frame.operand_stack.push(Value::Reference(string.string));
        }
        Const::Integer(int) => {
            cur_frame.operand_stack.push(Value::Int(int.int));
        }
        _ => panic!("unsupported operation")
    }
}

/// astore_<n>
///
/// See [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.astore_n).
pub fn astore_n(thread: &mut Thread, n: u16) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let value = cur_frame.operand_stack.pop();
    match &value {
        Value::Reference(_) | Value::ReturnAddress(_) => {},
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
    let class_name = method.class.name.clone();

    // Load the class if not loaded.
    let class = thread.runtime.method_area.insert(thread.runtime.heap.clone(), method.class.name.as_str());

    // Does the class need to be initialized by us?
    let send_init = thread.runtime.method_area.try_start_initialize(method.class.name.as_str());
    if send_init.is_some() {
        // We need to run the full <clinit> of the class first!
        let clinit_method = thread.runtime.method_area.find_method(method.class.name.as_str(), "<clinit>", &MethodType::from_descriptor("()V").unwrap());
        let mut clinit_thread = Thread::new(thread.runtime.clone(), class.const_pool.clone(), clinit_method);

        // Run <clinit> on another thread
        let clinit_handle = spawn(move|| clinit_thread.run());
        clinit_handle.join().unwrap();

        // Mark as initialized.
        send_init.unwrap().send(()).unwrap();
    }

    // We can call the static method now
    let method = thread.runtime.method_area.find_method(method.class.name.as_str(), method.name.as_str(), &method.descriptor);

    if !method.is_static {
        panic!("Expected a static method");
    }

    let args: Vec<Value> = (0..method.descriptor.parameters.len())
        .map(|_| cur_frame.operand_stack.pop())
        .rev()
        .collect();

    if method.is_native {
        let func = thread.runtime.native.find(class_name.as_str(), method.name.as_str(), &method.descriptor).unwrap();

        let result = func(thread.runtime.clone(), args);
        if let Some(result) = result {
            cur_frame.operand_stack.push(result);
        }
    } else {
        panic!("not implemented");
    }
}
use std::sync::Arc;
use std::thread::spawn;
use crate::java::{MethodType, Value};
use crate::runtime::{Method, Runtime};
use crate::runtime::method_area::Field;
use crate::thread::Thread;

/// Many instructions involve resolving class types, which follows a strict algorithm to ensure that
/// classes are loaded in the correct order.
///
/// This function handles this process.
pub fn resolve_class(runtime: Arc<Runtime>, class: &str) {

    // Load class, and all it's superclasses into the method area.
    let (class, _) = runtime.method_area.insert(runtime.clone(), class);

    // Classes that this thread needs to initialize.
    let mut to_init = runtime.method_area.try_start_init(class.name.as_str());

    let classes = to_init.iter().map(|t| t.0.clone()).collect();
    let mut clinit_thread = Thread::clinit(runtime.clone(), classes);

    spawn(move || clinit_thread.run()).join().unwrap();

    for (_, sender) in to_init {
        sender.send(()).unwrap();
    }

    // Also want to resolve all the field & method types in the class
    for field in class.fields.iter() {
        resolve_field(runtime.clone(), field);
    }

    // And resolve all the methods!
    for method in class.methods.iter() {
        resolve_method(runtime.clone(), method);
    }
}

/// Like resolving a class, but we need to resolve every type that is in the method signature!
pub fn resolve_method(runtime: Arc<Runtime>, method: &Arc<Method>) {
    for class in method.descriptor.class_names() {
        resolve_class(runtime.clone(), class.as_str())
    }
}

pub fn resolve_field(runtime: Arc<Runtime>, field: &Arc<Field>) {
    for class in field.descriptor.class_name() {
        resolve_class(runtime.clone(), class.as_str())
    }
}

/// Instruction `new` creates a new object in the heap.
///
/// Opcode 0xBB
///
/// For further information, see [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.new).
pub fn new(thread: &mut Thread) {
    let mut cur_frame = thread.stack.last_mut().unwrap();

    let class_idx = cur_frame.read_u16();
    let class_const = cur_frame.const_pool.get_class(class_idx);
    resolve_class(thread.runtime.clone(), class_const.name.as_str());
    let (class, _) = thread.runtime.method_area.insert(thread.runtime.clone(), class_const.name.as_str());

    let new_ref = thread.runtime.heap.insert_new(&class);

    cur_frame.operand_stack.push(Value::Reference(new_ref));
}
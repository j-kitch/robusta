use crate::java::{Int, Reference, Value};
use crate::thread::Thread;

pub fn check_cast(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u16();

    let class = thread.runtime.method_area.resolve_class(frame.const_pool, index);
    let class = unsafe { class.as_ref().unwrap() };
    let object_ref = frame.operand_stack.pop().reference();

    if object_ref.0 == 0 {
        frame.operand_stack.push(Value::Reference(Reference(0)));
    }

    let object = thread.runtime.heap.get_object(object_ref);
    let object_class = object.class();

    if object_class.is_instance_of(class) {
        frame.operand_stack.push(Value::Reference(object_ref));
    } else {
        panic!("Checkcast failed");
    };
}

pub fn instance_of(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u16();

    let class = thread.runtime.method_area.resolve_class(frame.const_pool, index);
    let class = unsafe { class.as_ref().unwrap() };
    let object_ref = frame.operand_stack.pop().reference();

    if object_ref.0 == 0 {
        frame.operand_stack.push(Value::Int(Int(0)));
    }

    let object = thread.runtime.heap.get_object(object_ref);
    let object_class = object.class();

    let result = if object_class.is_instance_of(class) {
        Value::Int(Int(1))
    } else {
        Value::Int(Int(0))
    };

    frame.operand_stack.push(result);
}
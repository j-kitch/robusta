use crate::java::{Int, Reference, Value};
use crate::thread::Thread;

pub fn check_cast(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u16();

    let class = thread.runtime.method_area.resolve_class(frame.const_pool, index);
    let reference = frame.operand_stack.pop().reference();

    if reference.0 == 0 {
        frame.operand_stack.push(Value::Reference(Reference(0)));
        return;
    }

    let heaped = thread.runtime.heap.get(reference);

    if heaped.class().is_instance_of(&class) {
        frame.operand_stack.push(Value::Reference(reference));
    } else {
        panic!("Checkcast failed");
    };
}

pub fn instance_of(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let index = frame.read_u16();

    let class = thread.runtime.method_area.resolve_class(frame.const_pool, index);
    let reference = frame.operand_stack.pop().reference();

    if reference.0 == 0 {
        frame.operand_stack.push(Value::Int(Int(0)));
        return;
    }

    let heaped = thread.runtime.heap.get(reference);
    let this_class = heaped.class();

    let result = if this_class.is_instance_of(&class) {
        Value::Int(Int(1))
    } else {
        Value::Int(Int(0))
    };

    frame.operand_stack.push(result);
}
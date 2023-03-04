use crate::java::Value;
use crate::thread::Thread;

pub fn get_field(thread: &mut Thread) {
    let curr_frame = thread.stack.last_mut().unwrap();

    let field_idx = curr_frame.read_u16();
    let field = curr_frame.const_pool.get_field(field_idx);

    let (_, _) = thread.runtime.method_area.insert(thread.runtime.clone(), field.class.name.as_str());

    let object_ref = curr_frame.operand_stack.pop();
    let object_ref = if let Value::Reference(reference) = object_ref {
        reference
    } else {
        panic!("Expected reference")
    };

    let object = thread.runtime.heap.load_object(object_ref);
    let field_value = object.get_field(field.as_ref());

    curr_frame.operand_stack.push(field_value);
}

pub fn put_field(thread: &mut Thread) {
    let curr_frame = thread.stack.last_mut().unwrap();

    let field_idx = curr_frame.read_u16();
    let field = curr_frame.const_pool.get_field(field_idx);

    let (_, _) = thread.runtime.method_area.insert(thread.runtime.clone(), field.class.name.as_str());

    let value = curr_frame.operand_stack.pop();

    let object_ref = curr_frame.operand_stack.pop();
    let object_ref = if let Value::Reference(reference) = object_ref {
        reference
    } else {
        panic!("Expected reference")
    };

    let object = thread.runtime.heap.load_object(object_ref);
    object.set_field(field.as_ref(), value)
}
use crate::java::Value;
use crate::thread::Thread;

pub fn get_field(thread: &mut Thread) {
    let curr_frame = thread.stack.last_mut().unwrap();

    let field_idx = curr_frame.read_u16();
    let field = curr_frame.const_pool.get_field(field_idx);

    let (class, _) = thread.runtime.method_area.insert(thread.runtime.clone(), field.class.name.as_str());

    let object_ref = curr_frame.operand_stack.pop();
    let object_ref = if let Value::Reference(reference) = object_ref {
        reference
    } else {
        panic!("Expected reference")
    };

    let object = thread.runtime.heap.load_object(object_ref);
    let heap_field_idx = class.find_field_idx(&field);

    let field = &object.fields.get(heap_field_idx).unwrap();
    let field_value = field.get_value();

    curr_frame.operand_stack.push(field_value);
}

pub fn put_field(thread: &mut Thread) {
    let curr_frame = thread.stack.last_mut().unwrap();

    let field_idx = curr_frame.read_u16();
    let field = curr_frame.const_pool.get_field(field_idx);

    let (class, _) = thread.runtime.method_area.insert(thread.runtime.clone(), field.class.name.as_str());

    let value = curr_frame.operand_stack.pop();

    let object_ref = curr_frame.operand_stack.pop();
    let object_ref = if let Value::Reference(reference) = object_ref {
        reference
    } else {
        panic!("Expected reference")
    };

    let object = thread.runtime.heap.load_object(object_ref);
    let heap_field_idx = class.find_field_idx(&field);

    let field = &object.fields[heap_field_idx];
    field.set_value(value);
}
use crate::method_area::const_pool::FieldKey;
use crate::thread::Thread;

pub fn get_field(thread: &mut Thread) {
    let curr_frame = thread.stack.last_mut().unwrap();

    let field_idx = curr_frame.read_u16();
    let field = thread.runtime.method_area.resolve_field(thread.runtime.clone(), curr_frame.const_pool, field_idx);
    let field = unsafe { field.as_ref().unwrap() };
    let class = unsafe { field.class.as_ref().unwrap() };

    let obj_ref = curr_frame.operand_stack.pop_cat_one().reference();
    let object = thread.runtime.heap.get_object(obj_ref);

    let field_value = object.get_field(&FieldKey {
        class: class.name.clone(),
        name: field.name.clone(),
        descriptor: field.descriptor.clone(),
    }).cat_one();

    curr_frame.operand_stack.push_cat_one(field_value);
}

pub fn put_field(thread: &mut Thread) {
    let curr_frame = thread.stack.last_mut().unwrap();

    let field_idx = curr_frame.read_u16();
    let field = thread.runtime.method_area.resolve_field(thread.runtime.clone(), curr_frame.const_pool, field_idx);
    let field = unsafe { field.as_ref().unwrap() };
    let class = unsafe { field.class.as_ref().unwrap() };

    let value = curr_frame.operand_stack.pop_cat_one();

    let obj_ref = curr_frame.operand_stack.pop_cat_one().reference();
    let object = thread.runtime.heap.get_object(obj_ref);

    object.set_field(&FieldKey {
        class: class.name.clone(),
        name: field.name.clone(),
        descriptor: field.descriptor.clone(),
    }, value)
}
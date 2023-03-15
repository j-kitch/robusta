use tracing::trace;
use crate::log;
use crate::method_area::const_pool::FieldKey;
use crate::thread::Thread;

pub fn get_field(thread: &mut Thread) {
    let curr_frame = thread.stack.last_mut().unwrap();
    let const_pool = curr_frame.const_pool;

    let field_idx = curr_frame.read_u16();
    let field = thread.runtime.method_area.resolve_field(thread.runtime.clone(), const_pool, field_idx);
    let field = unsafe { field.as_ref().unwrap() };
    let class = unsafe { field.class.as_ref().unwrap() };

    let curr_frame = thread.stack.last_mut().unwrap();
    let obj_ref = curr_frame.operand_stack.pop().reference();
    let object = thread.runtime.heap.get_object(obj_ref);

    let field_value = object.get_field(&FieldKey {
        class: class.name.clone(),
        name: field.name.clone(),
        descriptor: field.descriptor.clone(),
    });

    curr_frame.operand_stack.push_value(field_value);
}

pub fn get_static(thread: &mut Thread) {
    let curr_frame = thread.stack.last_mut().unwrap();
    let const_pool = curr_frame.const_pool;

    let field_idx = curr_frame.read_u16();

    trace!(target: log::INSTR, pc=curr_frame.pc - 3, opcode="getstatic", index=field_idx);

    let field = thread.runtime.method_area.resolve_static(thread.runtime.clone(), const_pool, field_idx);
    let field = unsafe { field.as_ref().unwrap() };
    let class = unsafe { field.class.as_ref().unwrap() };

    let static_ref = thread.runtime.heap.get_static(class);
    let static_obj = thread.runtime.heap.get_object(static_ref);

    let field_value = static_obj.get_static(&FieldKey {
        class: class.name.clone(),
        name: field.name.clone(),
        descriptor: field.descriptor.clone(),
    });

    let curr_frame = thread.stack.last_mut().unwrap();
    curr_frame.operand_stack.push_value(field_value);
}

pub fn put_static(thread: &mut Thread) {
    let curr_frame = thread.stack.last_mut().unwrap();
    let const_pool = curr_frame.const_pool;

    let field_idx = curr_frame.read_u16();

    trace!(target: log::INSTR, pc=curr_frame.pc - 3, opcode="putstatic", index=field_idx);

    let field = thread.runtime.method_area.resolve_static(thread.runtime.clone(), const_pool, field_idx);
    let field = unsafe { field.as_ref().unwrap() };
    let class = unsafe { field.class.as_ref().unwrap() };

    let static_ref = thread.runtime.heap.get_static(class);
    let static_obj = thread.runtime.heap.get_object(static_ref);

    let curr_frame = thread.stack.last_mut().unwrap();
    let value = curr_frame.operand_stack.pop();

    static_obj.set_static(&FieldKey {
        class: class.name.clone(),
        name: field.name.clone(),
        descriptor: field.descriptor.clone(),
    }, value.cat_one());
}

pub fn put_field(thread: &mut Thread) {
    let curr_frame = thread.stack.last_mut().unwrap();
    let const_pool = curr_frame.const_pool;

    let field_idx = curr_frame.read_u16();

    trace!(target: log::INSTR, pc=curr_frame.pc - 3, opcode="putfield", index=field_idx);

    let field = thread.runtime.method_area.resolve_field(thread.runtime.clone(), const_pool, field_idx);
    let field = unsafe { field.as_ref().unwrap() };
    let class = unsafe { field.class.as_ref().unwrap() };

    let curr_frame = thread.stack.last_mut().unwrap();
    let value = curr_frame.operand_stack.pop().cat_one();

    let obj_ref = curr_frame.operand_stack.pop().reference();
    let object = thread.runtime.heap.get_object(obj_ref);

    object.set_field(&FieldKey {
        class: class.name.clone(),
        name: field.name.clone(),
        descriptor: field.descriptor.clone(),
    }, value)
}
use crate::class::Const;
use crate::thread::Thread;

pub fn put(thread: &mut Thread) {
    let runtime = thread.rt.as_ref().borrow_mut();
    let current = thread.frames.current_mut();

    let field_idx = current.read_u16();
    let field_const = match current.class.const_pool.get(&field_idx).unwrap() {
        Const::Field(field) => field,
        _ => panic!("err"),
    };

    let value = current.op_stack.pop_value(&field_const.descriptor);
    let obj_ref = current.op_stack.pop_ref();

    let obj = runtime.heap.get(obj_ref);
    let mut obj = obj.as_ref().borrow_mut();
    let obj = obj.obj_mut();

    let field = obj.fields.iter_mut()
        .find(|f| f.field.name.eq(&field_const.name) && f.field.descriptor.eq(&field_const.descriptor))
        .unwrap();

    field.value = value;
}

pub fn get(thread: &mut Thread) {
    let runtime = thread.rt.as_ref().borrow_mut();
    let current = thread.frames.current_mut();

    let field_idx = current.read_u16();
    let field_const = match current.class.const_pool.get(&field_idx).unwrap() {
        Const::Field(field) => field,
        _ => panic!("err"),
    };

    let obj_ref = current.op_stack.pop_ref();

    let obj = runtime.heap.get(obj_ref);
    let mut obj = obj.as_ref().borrow_mut();
    let obj = obj.obj_mut();

    let field = obj.fields.iter_mut()
        .find(|f| f.field.name.eq(&field_const.name) && f.field.descriptor.eq(&field_const.descriptor))
        .unwrap();

    let value = field.value;

    current.op_stack.push(value);
}

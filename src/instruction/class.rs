use std::borrow::BorrowMut;
use crate::class::Const;
use crate::shim;

use crate::thread::Thread;

pub fn new(thread: &mut Thread) {
    let mut runtime = thread.rt.as_ref().borrow_mut();
    let current = thread.frames.current_mut();

    let idx = current.read_u16();
    let class_name = match current.class.const_pool.get(&idx).unwrap() {
        Const::Class(class) => &class.name,
        _ => panic!("err")
    };
    let class = runtime.class_loader.borrow_mut().load(class_name).unwrap();
    let uninit_parents = runtime.class_loader.uninit_parents(&class.this_class);
    if !uninit_parents.is_empty() {
        current.pc -= 3;
        thread.frames.push(shim::init_parents_frame(&uninit_parents));
        return;
    }

    let (obj_ref, _) = runtime.heap.create(class.clone());

    current.op_stack.push_ref(obj_ref)
}

pub fn get_static(thread: &mut Thread) {
    let mut runtime = thread.rt.as_ref().borrow_mut();
    let current = thread.frames.current_mut();

    let idx = current.read_u16();
    let field_const = match current.class.const_pool.get(&idx).unwrap() {
        Const::Field(field_ref) => field_ref,
        _ => panic!("err")
    };

    let class = runtime.class_loader.borrow_mut().load(&field_const.class).unwrap();
    let uninit_parents = runtime.class_loader.uninit_parents(&class.this_class);
    if !uninit_parents.is_empty() {
        current.pc -= 3;
        thread.frames.push(shim::init_parents_frame(&uninit_parents));
        return;
    }

    let value_idx = class.get_static_field_idx(&field_const.name, &field_const.descriptor).unwrap();
    let value = runtime.class_loader.get_static(&field_const.class, value_idx);

    current.op_stack.push(value)
}

pub fn put_static(thread: &mut Thread) {
    let mut runtime = thread.rt.as_ref().borrow_mut();
    let current = thread.frames.current_mut();

    let idx = current.read_u16();
    let field_const = match current.class.const_pool.get(&idx).unwrap() {
        Const::Field(field_ref) => field_ref,
        _ => panic!("err")
    };

    let class = runtime.class_loader.borrow_mut().load(&field_const.class).unwrap();
    let uninit_parents = runtime.class_loader.uninit_parents(&class.this_class);
    if !uninit_parents.is_empty() {
        current.pc -= 3;
        thread.frames.push(shim::init_parents_frame(&uninit_parents));
        return;
    }

    let value_idx = class.get_static_field_idx(&field_const.name, &field_const.descriptor);
    let value = current.op_stack.pop_value(&field_const.descriptor);

    runtime.class_loader.put_static(&field_const.class, value_idx.unwrap(), value)
}

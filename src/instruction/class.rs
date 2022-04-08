use std::borrow::BorrowMut;
use std::ops::Deref;

use crate::class::Const;
use crate::heap::Ref;
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

pub fn do_if_instance<F, G, H>(thread: &mut Thread, do_if_null: F, do_if_instance: G, do_if_not: H)
    where F: Fn(&mut Thread),
          G: Fn(&mut Thread, u32),
          H: Fn(&mut Thread, u32) {
    let current = thread.frames.current_mut();
    let type_idx = current.read_u16();
    let type_const = match current.class.const_pool.get(&type_idx).unwrap() {
        Const::Class(class_ref) => class_ref,
        _ => panic!("Not implemented instace_of for this!")
    };
    let object_ref = current.op_stack.pop_ref();

    if object_ref == 0 {
        return do_if_null(thread);
    }

    let mut is_instance = false;

    {
        // TODO: Standardise this!
        let mut runtime = thread.rt.as_ref().borrow_mut();
        let class = runtime.class_loader.borrow_mut().load(&type_const.name).unwrap();
        let uninit_parents = runtime.class_loader.uninit_parents(&class.this_class);
        if !uninit_parents.is_empty() {
            current.pc -= 3;
            current.op_stack.push_ref(object_ref);
            thread.frames.push(shim::init_parents_frame(&uninit_parents));
            return;
        }

        let obj = runtime.load_object(object_ref);
        let obj = obj.as_ref().borrow();
        is_instance = match obj.deref() {
            Ref::Obj(obj) => {
                // TODO: Assuming T is class, not interface
                let s = obj.class.clone();
                let t = class;
                s.is_sub_class_of(&t)
            }
            Ref::Arr(_) => {
                panic!("Not implemented is_instance for arrays yet!");
            }
        };
    }

    if is_instance {
        return do_if_instance(thread, object_ref);
    } else {
        return do_if_not(thread, object_ref);
    }
}

pub fn check_cast(thread: &mut Thread) {
    do_if_instance(
        thread,
        |t| {
            let current = t.frames.current_mut();
            current.op_stack.push_ref(0);
        },
        |t, obj_ref| {
            let current = t.frames.current_mut();
            current.op_stack.push_ref(obj_ref);
        },
        |_, _| {
            // TODO: Exceptions!
            panic!("ClassCastException")
        })
}

pub fn instance_of(thread: &mut Thread) {
    do_if_instance(
        thread,
        |t| {
            let current = t.frames.current_mut();
            current.op_stack.push_int(0);
        },
        |t, _| {
            let current = t.frames.current_mut();
            current.op_stack.push_int(1);
        },
        |t, _| {
            let current = t.frames.current_mut();
            current.op_stack.push_int(0);
        })
}

use std::borrow::BorrowMut;
use crate::class::Const;
use crate::shim;

use crate::thread::Thread;

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

    let value = class.get_static_field(&field_const.name, &field_const.descriptor).unwrap();

    current.op_stack.push(value)
}

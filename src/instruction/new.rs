use std::borrow::BorrowMut;
use crate::class::Const;
use crate::descriptor::Descriptor;
use crate::shim;
use crate::thread::Thread;

pub fn ref_arr(thread: &mut Thread) {
    let mut runtime = thread.rt.as_ref().borrow_mut();
    let current = thread.frames.current_mut();

    let class_idx = current.read_u16();
    let class_name = match current.class.const_pool.get(&class_idx).unwrap() {
        Const::Class(class) => &class.name,
        _ => panic!("err"),
    };

    // TODO: We need to standardise how we handle this scenario so we stop duplicating code!
    let class = runtime.class_loader.borrow_mut().load(class_name).unwrap();
    let uninit_parents = runtime.class_loader.uninit_parents(&class.this_class);
    if !uninit_parents.is_empty() {
        current.pc -= 3;
        thread.frames.push(shim::init_parents_frame(&uninit_parents));
        return;
    }

    let arr_count = current.op_stack.pop_int();

    let arr_ref = runtime.heap.borrow_mut().create_array(Descriptor::Object(class_name.to_string()), arr_count);

    current.op_stack.push_ref(arr_ref);
}

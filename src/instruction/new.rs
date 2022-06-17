use std::borrow::BorrowMut;

use crate::descriptor::Descriptor;
use crate::robusta::class::object::Const;
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
    let uninit_parents = runtime.class_loader.uninit_parents(&class.name());
    if !uninit_parents.is_empty() {
        current.pc -= 3;
        thread.frames.push(shim::init_parents_frame(&uninit_parents));
        return;
    }

    let arr_count = current.op_stack.pop_int();

    let arr_ref = runtime.heap.borrow_mut().create_array(Descriptor::parse(&class.descriptor()), arr_count);

    current.op_stack.push_ref(arr_ref);
}

pub fn multi_a_new_array(thread: &mut Thread) {
    let mut runtime = thread.rt.as_ref().borrow_mut();
    let current = thread.frames.current_mut();
    let index = current.read_u16();
    let dimensions = current.read_u8();

    let class = match current.class.const_pool.get(&index).unwrap() {
        Const::Class(class) => class.name.clone(),
        _ => panic!("err")
    };

    let counts: Vec<i32> = (1..dimensions)
        .map(|_| current.op_stack.pop_int())
        .collect();

    let class = runtime.load_class(&class);
    let descriptor = Descriptor::parse(class.as_ref().descriptor().as_str());

    let arr_ref = runtime.heap.create_multi_ref_array(descriptor, counts);

    current.op_stack.push_ref(arr_ref);
}

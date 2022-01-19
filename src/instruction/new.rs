use crate::class::Const;
use crate::shim;
use crate::thread::Thread;

pub fn ref_array(thread: &mut Thread) {
    let curr = thread.frames.current_mut();
    let mut runtime = thread.rt.borrow_mut();
    let idx = curr.read_u16();
    let class = match curr.class.const_pool.get(&idx).unwrap() {
        Const::Class(class) => class,
        _ => panic!("err"),
    };

    let class = runtime.class_loader.load(&class.name).unwrap();
    let uninit_parents = runtime.class_loader.uninit_parents(&class.this_class);
    if !uninit_parents.is_empty() {
        curr.pc -= 3;
        thread.frames.push(shim::init_parents_frame(&uninit_parents));
        return;
    }

    let count = curr.op_stack.pop_int() as usize;
    let array = vec![0; count];
    let arr_ref = runtime.heap.insert_ref_array(array);

    curr.op_stack.push_ref(arr_ref);
}

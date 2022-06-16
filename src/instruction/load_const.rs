use crate::shim;
use crate::robusta::class::object;
use crate::thread::Thread;

pub fn category_1(thread: &mut Thread) {
    let mut runtime = thread.rt.as_ref().borrow_mut();
    let current = thread.frames.current_mut();
    let index = current.read_u8() as u16;
    let cp_value = current.class.const_pool.get(&index).unwrap();
    match cp_value {
        object::Const::Int(i) => {
            current.op_stack.push_int(i.int);
        }
        object::Const::Float(f) => {
            current.op_stack.push_float(f.float);
        }
        object::Const::String(s) => {
            let reference = runtime.insert_str_const(&s.string);
            current.op_stack.push_ref(reference);
        }
        object::Const::Class(c) => {
            let class = runtime.class_loader.load(&c.name).unwrap();
            let uninit_parents = runtime.class_loader.uninit_parents(&c.name);
            if !uninit_parents.is_empty() {
                current.pc -= 2;
                thread.frames.push(shim::init_parents_frame(&uninit_parents));
                return;
            }
            let class_inst = runtime.create_class_object(class);
            current.op_stack.push_ref(class_inst);
        }
        x => panic!("{:?} I do not handle this yet :(", x)
    }
}

pub fn category_1_wide(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let index = current.read_u16();
    let cp_value = current.class.const_pool.get(&index).unwrap();
    match cp_value {
        object::Const::Int(i) => {
            current.op_stack.push_int(i.int);
        }
        object::Const::Float(f) => {
            current.op_stack.push_float(f.float);
        }
        _ => panic!("err")
    }
}

pub fn category_2_wide(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let index = current.read_u16();
    let cp_value = current.class.const_pool.get(&index).unwrap();
    match cp_value {
        object::Const::Long(l) => {
            current.op_stack.push_long(l.long);
        }
        object::Const::Double(d) => {
            current.op_stack.push_double(d.double);
        }
        _ => panic!("err")
    }
}

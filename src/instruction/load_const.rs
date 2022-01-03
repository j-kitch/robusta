use crate::class;
use crate::thread::Thread;

pub fn category_1(thread: &mut Thread) {
    let mut runtime = thread.rt.as_ref().borrow_mut();
    let current = thread.frames.current_mut();
    let index = current.read_u8() as u16;
    let cp_value = current.class.const_pool.get(&index).unwrap();
    match cp_value {
        class::Const::Int(i) => {
            current.op_stack.push_int(i.int);
        }
        class::Const::Float(f) => {
            current.op_stack.push_float(f.float);
        }
        class::Const::String(s) => {
            let reference = runtime.insert_str_const(&s.string);
            current.op_stack.push_ref(reference);
        }
        _ => panic!("err")
    }
}

pub fn category_1_wide(thread: &mut Thread) {
    let current = thread.frames.current_mut();
    let index = current.read_u16();
    let cp_value = current.class.const_pool.get(&index).unwrap();
    match cp_value {
        class::Const::Int(i) => {
            current.op_stack.push_int(i.int);
        }
        class::Const::Float(f) => {
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
        class::Const::Long(l) => {
            current.op_stack.push_long(l.long);
        }
        class::Const::Double(d) => {
            current.op_stack.push_double(d.double);
        }
        _ => panic!("err")
    }
}

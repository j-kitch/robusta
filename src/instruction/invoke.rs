use std::ops::DerefMut;

use crate::class::Const;
use crate::descriptor::Descriptor;
use crate::heap::Value;
use crate::thread::{Frame, Thread};
use crate::thread::local_vars::LocalVars;
use crate::thread::op_stack::OperandStack;

pub fn invoke_virtual(thread: &mut Thread) {
    let mut runtime = thread.rt.borrow_mut();
    let current = thread.frames.current_mut();
    let idx = current.read_u16();
    let method_const = match current.class.const_pool.get(&idx).unwrap() {
        Const::Method(method) => method,
        _ => panic!("err"),
    };

    let class = runtime.class_loader.load(&method_const.class).unwrap();
    let method = class.methods.iter()
        .find(|m| m.name.eq(&method_const.name) && m.descriptor.eq(&method_const.descriptor))
        .unwrap()
        .clone();

    let mut args = vec![];
    for arg in method.descriptor.args.iter().rev() {
        match arg {
            Descriptor::Boolean | Descriptor::Byte | Descriptor::Char | Descriptor::Short | Descriptor::Int => {
                args.push(Value::Int(current.op_stack.pop_int()));
            }
            Descriptor::Long => {
                args.push(Value::Long(current.op_stack.pop_long()));
            }
            Descriptor::Float => {
                args.push(Value::Float(current.op_stack.pop_float()));
            }
            Descriptor::Double => {
                args.push(Value::Double(current.op_stack.pop_double()));
            }
            Descriptor::Object(_) | Descriptor::Array(_) => {
                args.push(Value::Ref(current.op_stack.pop_ref()));
            }
        }
    }
    args.push( Value::Ref(current.op_stack.pop_ref()));
    args.reverse();

    if method.native {
        let func = runtime.native.find_method(&class.this_class, &method.name, &method.descriptor);
        let result = func(runtime.deref_mut(), args);
        if method.descriptor.returns.is_some() {
            current.op_stack.push(result.unwrap());
        }
    } else {
        let mut frame = Frame {
            pc: 0,
            class,
            local_vars: LocalVars::new(method.max_locals),
            op_stack: OperandStack::new(method.max_stack),
            method,
        };
        let mut idx = 0;
        for arg in args.iter() {
            match arg {
                Value::Int(i) => {
                    frame.local_vars.store_int(idx, i.clone());
                    idx += 1;
                }
                Value::Long(l) => {
                    frame.local_vars.store_long(idx, l.clone());
                    idx += 2;
                }
                Value::Float(f) => {
                    frame.local_vars.store_float(idx, f.clone());
                    idx += 1;
                }
                Value::Double(d) => {
                    frame.local_vars.store_double(idx, d.clone());
                    idx += 2;
                }
                Value::Ref(r) => {
                    frame.local_vars.store_ref(idx, r.clone());
                    idx += 1;
                }
            }
        }
        thread.frames.push(frame);
    }
}

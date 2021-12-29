use std::cell::RefCell;
use std::env;
use std::rc::Rc;

use robusta::descriptor::MethodDescriptor;
use robusta::runtime::Runtime;
use robusta::thread::{Frame, Thread};
use robusta::thread::local_vars::{Locals, LocalVars};
use robusta::thread::op_stack::OperandStack;

fn main() {
    let main_class_name = env::args().nth(1).unwrap()
        .replace(".", "/");
    let args: Vec<String> = env::args().skip(2).collect();

    let mut rt = Runtime::new();

    let args_arr = args.iter()
        .map(|arg| rt.insert_str_const(arg))
        .collect();
    let args_arr_ref = rt.insert_ref_array(args_arr);

    let class = rt.load_class(&main_class_name);
    let main = class
        .find_method("main", &MethodDescriptor::parse("([Ljava/lang/String;)V"))
        .unwrap();

    let mut thread = Thread {
        rt: Rc::new(RefCell::new(rt)),
        frames: vec![
            Frame {
                pc: 0,
                class: class.clone(),
                local_vars: LocalVars::new(main.max_locals.clone()),
                op_stack: OperandStack::new(main.max_stack.clone()),
                method: main,
            }
        ],
    };

    thread.store_ref(0, args_arr_ref);
    thread.run();
}

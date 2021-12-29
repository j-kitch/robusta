use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::env;
use std::ops::DerefMut;
use std::rc::Rc;

use robusta::class_loader::ClassLoader;
use robusta::descriptor::MethodDescriptor;
use robusta::heap::{Heap, Value};
use robusta::heap::Ref::Obj;
use robusta::native::NativeMethods;
use robusta::runtime::Runtime;
use robusta::thread::{Frame, Thread};
use robusta::thread::local_vars::{Locals, LocalVars};
use robusta::thread::op_stack::OperandStack;

fn main() {
    let main_class_name = env::args().nth(1).unwrap()
        .replace(".", "/");
    let args: Vec<String> = env::args().skip(2).collect();
    let mut loader = ClassLoader::new();
    let mut heap = Heap::new();
    let string_class = loader.load("java/lang/String").unwrap();

    let args_arr = args.iter().map(|arg| {
        let (str_ref, mut str_obj) = heap.create(string_class.clone());
        let str_obj = str_obj.borrow_mut();
        let str_obj = str_obj.as_ref();
        let mut str_obj = str_obj.borrow_mut();
        let str_obj = match str_obj.deref_mut() {
            Obj(obj) => obj,
            _ => panic!("not an array")
        };

        let mut field = str_obj.fields.iter_mut()
            .find(|f| f.field.name.eq("chars"))
            .unwrap();

        let utf16: Vec<u16> = arg.encode_utf16().collect();
        let chars_ref = heap.insert_char_array(utf16);
        field.value = Value::Ref(chars_ref);
        str_ref
    }).collect();

    let args_arr_ref = heap.insert_ref_array(args_arr);

    let class = loader.load(&main_class_name);
    if class.is_none() {
        eprintln!("Error: Could not find or load main class {}", &main_class_name);
        std::process::exit(1);
    }
    let class = class.unwrap();
    let main = class.as_ref()
        .find_method("main", &MethodDescriptor::parse("([Ljava/lang/String;)V"))
        .unwrap();

    let mut thread = Thread {
        rt: Rc::from(RefCell::from(Runtime {
            class_loader: loader,
            heap,
            native: NativeMethods::load(),
        })),
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

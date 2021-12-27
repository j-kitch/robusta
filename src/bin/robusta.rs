use std::{env, format};
use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::Read;
use std::ops::DerefMut;

use robusta::class::Class;
use robusta::class_file::Reader;
use robusta::class_loader::ClassLoader;
use robusta::heap::{Heap, Object, Value};
use robusta::heap::Ref::Obj;
use robusta::thread::{Frame, Thread};
use robusta::thread::local_vars::LocalVars;
use robusta::thread::op_stack::OperandStack;

fn main() {
    let main_class_name = env::args().nth(1).unwrap()
        .replace(".", "/");
    let args: Vec<String> = env::args().skip(2).collect();
    let mut loader = ClassLoader::new();
    let mut heap = Heap::new();
    let stringClass = loader.load("java/lang/String").unwrap();

    let args_arr = args.iter().map(|arg| {
        let (strRef, mut strObject) = heap.create(stringClass.clone());
        let mut strObject = strObject.borrow_mut();
        let mut strObject = strObject.as_ref();
        let mut strObject = strObject.borrow_mut();
        let mut strObject = match strObject.deref_mut() {
            Obj(obj) => obj,
            _ => panic!("not an array")
        };

        let mut field = strObject.fields.iter_mut()
            .find(|f| f.field.name.eq("chars"))
            .unwrap();

        let utf16: Vec<u16> = arg.encode_utf16().collect();
        let chars_ref = heap.insert_char_array(utf16);
        field.value = Value::Ref(chars_ref);
        strRef
    }).collect();

    let args_arr_ref = heap.insert_ref_array(args_arr);

    let mut class = loader.load(&main_class_name);
    if class.is_none() {
        eprintln!("Error: Could not find or load main class {}", &main_class_name);
        std::process::exit(1);
    }
    let class = class.unwrap();
    let main = class.as_ref()
        .find_method("main", "([Ljava/lang/String;)V")
        .unwrap();
    let mut thread = Thread {
        frames: vec![
            Frame {
                pc: 0,
                class: class.clone(),
                local_vars: LocalVars::new(main.max_locals.clone()),
                op_stack: OperandStack::new(main.max_stack.clone()),
                method: main,
            }
        ]
    };

    thread.frames.last_mut().unwrap().local_vars.store_ref(0, args_arr_ref);

    thread.run();
}

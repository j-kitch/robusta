use std::env;

use robusta::descriptor::MethodDescriptor;
use robusta::heap::Value;
use robusta::runtime::Runtime;
use robusta::thread::Thread;

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

    let mut thread = Thread::new(rt);
    thread.create_frame(class, main, vec![Value::Ref(args_arr_ref)]);
    thread.run();
}

use std::collections::HashMap;
use std::sync::Arc;
use std::thread::spawn;

use crate::class_file::Code;
use crate::java::{MethodType, Value};
use crate::native::{Method, Plugin};
use crate::native::stateless::stateless;
use crate::runtime;
use crate::runtime::{ConstPool, Runtime};
use crate::thread::Thread;

pub fn java_lang_plugins() -> Vec<Box<dyn Plugin>> {
    vec![
        stateless(
            Method {
                class: "java.lang.String".to_string(),
                name: "intern".to_string(),
                descriptor: MethodType::from_descriptor("()Ljava/lang/String;").unwrap(),
            },
            Arc::new(string_intern),
        ),
        stateless(
            Method {
                class: "java.lang.Object".to_string(),
                name: "getClass".to_string(),
                descriptor: MethodType::from_descriptor("()Ljava/lang/Class;").unwrap(),
            },
            Arc::new(object_get_class),
        )
    ]
}


fn string_intern(runtime: Arc<Runtime>, values: Vec<Value>) -> Option<Value> {
    let string_ref = values[0].reference();

    let interned_string_ref = runtime.heap.intern_string(string_ref);

    Some(Value::Reference(interned_string_ref))
}

fn object_get_class(runtime: Arc<Runtime>, values: Vec<Value>) -> Option<Value> {
    let object_ref = values[0].reference();
    let object_obj = runtime.heap.load_object(object_ref);

    let class_name = &object_obj.class().name;

    intern_class(runtime, class_name)
}

pub fn intern_class(runtime: Arc<Runtime>, class_name: &str) -> Option<Value> {
    let (class_class, _) = runtime.method_area.insert(runtime.clone(), "java.lang.Class");
    let init_method = runtime.method_area.find_method("java.lang.Class", "<init>", &MethodType::from_descriptor("(Ljava/lang/String;)V").unwrap());

    let class_obj_ref = runtime.heap.insert_new(&class_class);

    let string_ref = runtime.heap.get_string(class_name);

    let mut thread = Thread::empty(runtime.clone());
    thread.add_frame("<robusta>".into(), Arc::new(ConstPool { pool: HashMap::new() }), Arc::new(runtime::Method {
        is_static: true,
        is_native: false,
        name: "<exit>".to_string(),
        descriptor: MethodType::from_descriptor("()V").unwrap(),
        code: Some(Code {
            max_stack: 0,
            max_locals: 0,
            code: vec![0xB1],
        }),
    }));
    thread.add_frame(class_class.name.clone(), class_class.const_pool.clone(), init_method);
    let frame = thread.stack.last_mut().unwrap();
    frame.local_vars.store_value(0, Value::Reference(class_obj_ref));
    frame.local_vars.store_value(1, Value::Reference(string_ref));

    spawn(move || thread.run()).join().unwrap();
    // println!("Foo");
    let class_obj_ref = runtime.heap.intern_class(class_name, class_obj_ref);
    // println!("Boo");

    Some(Value::Reference(class_obj_ref))
}
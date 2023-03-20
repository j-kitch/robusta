use std::sync::Arc;
use crate::java::{FieldType, Int, MethodType, Value};
use crate::method_area;
use crate::method_area::ObjectClass;
use crate::method_area::const_pool::{FieldKey, MethodKey};
use crate::native::{Args, Plugin};
use crate::native::stateless::{Method, stateless};

pub fn system_plugins() -> Vec<Arc<dyn Plugin>> {
    vec![
        stateless(
            Method {
                class: "java.lang.System".to_string(),
                name: "registerNatives".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(register_natives),
        ),
        stateless(
            Method {
                class: "java.lang.System".to_string(),
                name: "initProperties".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/util/Properties;)Ljava/util/Properties;").unwrap(),
            },
            Arc::new(init_properties),
        ),
        stateless(
            Method {
                class: "java.lang.System".to_string(),
                name: "setIn0".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/io/InputStream;)V").unwrap(),
            },
            Arc::new(set_in_0),
        ),
        stateless(
            Method {
                class: "java.lang.System".to_string(),
                name: "setOut0".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/io/PrintStream;)V").unwrap(),
            },
            Arc::new(set_out_0),
        ),
        stateless(
            Method {
                class: "java.lang.System".to_string(),
                name: "setErr0".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/io/PrintStream;)V").unwrap(),
            },
            Arc::new(set_err_0),
        ),
        stateless(
            Method {
                class: "sun.misc.Unsafe".to_string(),
                name: "getIntVolatile".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/Object;J)I").unwrap(),
            },
            Arc::new(get_int_volatile),
        )
    ]
}

fn register_natives(args: &Args) -> (Option<Value>, Option<Value>) {
    let system_class = args.runtime.method_area.load_class("java.lang.System");

    let init_method = system_class.find_method(&MethodKey {
        class: "java.lang.System".to_string(),
        name: "initializeSystemClass".to_string(),
        descriptor: MethodType::from_descriptor("()V").unwrap(),
    }).unwrap();

    let thread = unsafe { args.thread.cast_mut().as_mut().unwrap() };

    let (_, ex) = thread.native_invoke(&*system_class as *const ObjectClass, init_method as *const method_area::Method, vec![]);
    (None, ex)
}

fn init_properties(args: &Args) -> (Option<Value>, Option<Value>) {
    (Some(args.params[0]), None)
}

fn set_in_0(args: &Args) -> (Option<Value>, Option<Value>) {
    let input_stream = args.params[0].reference();

    let class = args.runtime.method_area.load_class("java.lang.System");
    let static_ref = args.runtime.heap.get_static(&class);
    let static_obj = args.runtime.heap.get_object(static_ref);

    static_obj.set_static(&FieldKey {
        class: "java.lang.System".to_string(),
        name: "in".to_string(),
        descriptor: FieldType::from_descriptor("Ljava.io.InputStream;").unwrap(),
    }, Value::Reference(input_stream));

    (None, None)
}

fn set_out_0(args: &Args) -> (Option<Value>, Option<Value>) {
    let print_stream = args.params[0].reference();

    let class = args.runtime.method_area.load_class("java.lang.System");
    let static_ref = args.runtime.heap.get_static(&class);
    let static_obj = args.runtime.heap.get_object(static_ref);

    static_obj.set_static(&FieldKey {
        class: "java.lang.System".to_string(),
        name: "out".to_string(),
        descriptor: FieldType::from_descriptor("Ljava.io.PrintStream;").unwrap(),
    }, Value::Reference(print_stream));

    (None, None)
}

fn set_err_0(args: &Args) -> (Option<Value>, Option<Value>) {
    let print_stream = args.params[0].reference();

    let class = args.runtime.method_area.load_class("java.lang.System");
    let static_ref = args.runtime.heap.get_static(&class);
    let static_obj = args.runtime.heap.get_object(static_ref);

    static_obj.set_static(&FieldKey {
        class: "java.lang.System".to_string(),
        name: "err".to_string(),
        descriptor: FieldType::from_descriptor("Ljava.io.PrintStream;").unwrap(),
    }, Value::Reference(print_stream));

    (None, None)
}

fn get_int_volatile(args: &Args) -> (Option<Value>, Option<Value>) {
    let object = args.params[1].reference();
    let offset = args.params[2].long().0;

    let object = args.runtime.heap.get_object(object);

    let value = unsafe {
        let ptr: *const i32 = object.data.add(offset as usize).cast();
        ptr.read_volatile()
    };

    (Some(Value::Int(Int(value))), None)
}
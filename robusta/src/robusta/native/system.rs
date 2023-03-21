use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

use maplit::hashmap;

use crate::java::{FieldType, Int, Long, MethodType, Reference, Value};
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
                name: "identityHashCode".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/Object;)I").unwrap(),
            },
            Arc::new(identity_hash_code),
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
                class: "java.lang.System".to_string(),
                name: "mapLibraryName".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/String;)Ljava/lang/String;").unwrap(),
            },
            Arc::new(map_library_name),
        ),
        stateless(
            Method {
                class: "java.lang.ClassLoader".to_string(),
                name: "findBuiltinLib".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/String;)Ljava/lang/String;").unwrap(),
            },
            Arc::new(find_builtin),
        ),
        stateless(
            Method {
                class: "sun.misc.Unsafe".to_string(),
                name: "getIntVolatile".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/Object;J)I").unwrap(),
            },
            Arc::new(get_int_volatile),
        ),
        stateless(
            Method {
                class: "sun.misc.Unsafe".to_string(),
                name: "allocateMemory".to_string(),
                descriptor: MethodType::from_descriptor("(J)J").unwrap(),
            },
            Arc::new(allocate_memory),
        ),
        stateless(
            Method {
                class: "sun.misc.Unsafe".to_string(),
                name: "freeMemory".to_string(),
                descriptor: MethodType::from_descriptor("(J)V").unwrap(),
            },
            Arc::new(free_memory),
        ),
        stateless(
            Method {
                class: "java.util.concurrent.atomic.AtomicLong".to_string(),
                name: "VMSupportsCS8".to_string(),
                descriptor: MethodType::from_descriptor("()Z").unwrap(),
            },
            Arc::new(vm_supports_cs8),
        ),
        stateless(
            Method {
                class: "sun.misc.Unsafe".to_string(),
                name: "getByte".to_string(),
                descriptor: MethodType::from_descriptor("(J)B").unwrap(),
            },
            Arc::new(get_byte),
        ),
        stateless(
            Method {
                class: "sun.misc.Unsafe".to_string(),
                name: "putLong".to_string(),
                descriptor: MethodType::from_descriptor("(JJ)V").unwrap(),
            },
            Arc::new(put_long),
        ),
        stateless(
            Method {
                class: "sun.reflect.NativeConstructorAccessorImpl".to_string(),
                name: "newInstance0".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object;").unwrap(),
            },
            Arc::new(new_instance),
        ),
        stateless(
            Method {
                class: "java.io.UnixFileSystem".to_string(),
                name: "getBooleanAttributes0".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/io/File;)I").unwrap(),
            },
            Arc::new(get_boolean_attributes_0),
        ),
        stateless(
            Method {
                class: "java.lang.ClassLoader$NativeLibrary".to_string(),
                name: "load".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/String;Z)V").unwrap(),
            },
            Arc::new(load_library),
        ),
        stateless(
            Method {
                class: "sun.misc.Signal".to_string(),
                name: "findSignal".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/String;)I").unwrap(),
            },
            Arc::new(find_signal),
        ),
        stateless(
            Method {
                class: "sun.misc.Signal".to_string(),
                name: "handle0".to_string(),
                descriptor: MethodType::from_descriptor("(IJ)J").unwrap(),
            },
            Arc::new(signal_handle_0),
        ),
        stateless(
            Method {
                class: "sun.misc.URLClassPath".to_string(),
                name: "getLookupCacheURLs".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/ClassLoader;)[Ljava/net/URL;").unwrap(),
            },
            Arc::new(lookup_cache_urls),
        ),
        // stateless(
        //     Method {
        //         class: "sun.misc.Unsafe".to_string(),
        //         name: "arrayIndexScale".to_string(),
        //         descriptor: MethodType::from_descriptor("(Ljava/lang/Class;)I").unwrap(),
        //     },
        //     Arc::new(array_index_scale),
        // ),
    ]
}

// fn array_index_scale(args: &Args) -> (Option<Value>, Option<Value>) {
//     let class = args.params[1].reference();
//     let class = args.runtime.heap.get_object(class);
//     let name = class.get_string("name", &args.runtime.heap);
//
//     let scale = match name.as_str() {
//         "[Z" | "[B" => 1,
//         "[C" | "[S" => 2,
//         "[J" | "[D" => 8,
//         _ => 4,
//     };
//
//     (Some(Value::Int(Int(scale))), None)
// }

fn signal_handle_0(_: &Args) -> (Option<Value>, Option<Value>) {
    (Some(Value::Long(Long(0))), None)
}

fn lookup_cache_urls(_: &Args) -> (Option<Value>, Option<Value>) {
    (Some(Value::Reference(Reference(0))), None)
}

fn find_signal(args: &Args) -> (Option<Value>, Option<Value>) {
    let signal = args.params[0].reference();
    let signal = args.runtime.heap.get_string(signal);

    let int = match signal.as_str() {
        "HUP" => signal_hook::consts::SIGHUP,
        "INT" => signal_hook::consts::SIGINT,
        "TERM" => signal_hook::consts::SIGTERM,
        _ => panic!("unknown signal {}", signal)
    };

    let int = int as i32;

    (Some(Value::Int(Int(int))), None)
}

fn load_library(args: &Args) -> (Option<Value>, Option<Value>) {
    // TODO: Just set loaded to true internally!
    let library = args.params[0].reference();
    let library = args.runtime.heap.get_object(library);

    library.set_field(&FieldKey {
        class: "java.lang.ClassLoader$NativeLibrary".to_string(),
        name: "loaded".to_string(),
        descriptor: FieldType::Boolean,
    }, Value::Int(Int(1)));

    (None, None)
}

fn get_boolean_attributes_0(args: &Args) -> (Option<Value>, Option<Value>) {
    let file_ref = args.params[1].reference();
    let file = args.runtime.heap.get_object(file_ref);

    let path_ref = file.get_ref("path");
    let path = args.runtime.heap.get_string(path_ref);

    let path = PathBuf::from(&path);

    let mut status = 0;
    if path.exists() {
        status += 0x1;
    }
    if path.is_file() {
        status += 0x2;
    }
    if path.is_dir() {
        status += 0x4;
    }
    if path.file_name().unwrap().to_str().unwrap().starts_with('.') {
        status += 0x8;
    }

    (Some(Value::Int(Int(status))), None)
}

fn get_byte(args: &Args) -> (Option<Value>, Option<Value>) {
    let address = args.params[1].long().0;
    let x = unsafe {
        let ptr = address as usize as *const i8;
        ptr.read()
    };
    (Some(Value::Int(Int(x as i32))), None)
}

fn put_long(args: &Args) -> (Option<Value>, Option<Value>) {
    let address = args.params[1].long().0;
    let x = args.params[2].long().0;
    unsafe {
        let ptr = address as usize as *mut i64;
        ptr.write(x);
    }
    (None, None)
}

fn allocate_memory(args: &Args) -> (Option<Value>, Option<Value>) {
    let bytes = args.params[1].long().0 as usize;

    let raw_ptr = args.runtime.heap.allocator.raw(bytes);

    let ptr = raw_ptr as usize;
    let ptr = ptr as i64;

    (Some(Value::Long(Long(ptr))), None)
}

fn free_memory(_: &Args) -> (Option<Value>, Option<Value>) {
    // TODO: Handle this manual raw memory properly!
    // let bytes = args.params[1].long().0 as usize;
    //
    // let raw_ptr = args.runtime.heap.allocator.raw(bytes);
    //
    // let ptr = unsafe { raw_ptr as usize };
    // let ptr = ptr as i64;
    //
    // (Some(Value::Long(Long(ptr))), None)
    (None, None)
}

fn vm_supports_cs8(_: &Args) -> (Option<Value>, Option<Value>) {
    (Some(Value::Int(Int(0))), None)
}

fn new_instance(args: &Args) -> (Option<Value>, Option<Value>) {
    let constr_ref = args.params[0].reference();
    let args_arr_ref = args.params[1].reference();

    let constr_obj = args.runtime.heap.get_object(constr_ref);

    let class_ref = constr_obj.get_ref("clazz");
    let class = args.runtime.heap.get_object(class_ref);
    let name = class.get_string("name", &args.runtime.heap);
    let class = args.runtime.method_area.load_outer_class(&name);
    let class = class.obj();
    let constr = if args_arr_ref.0 == 0 {
        class.find_method(&MethodKey {
            class: class.name.clone(),
            name: "<init>".to_string(),
            descriptor: MethodType::from_descriptor("()V").unwrap(),
        }).unwrap()
    } else {
        let param_types_ref = constr_obj.get_ref("parameterTypes");
        let param_types = args.runtime.heap.get_array(param_types_ref);
        let param_types: Vec<String> = (0..param_types.length().0)
            .map(|idx| param_types.get_element(Int(idx)).reference())
            .map(|class_ref| args.runtime.heap.get_object(class_ref))
            .map(|class_obj| class_obj.class().name.clone())
            .collect();
        let signature = format!("({})V", param_types.join(""));
        let constr = class.find_method(&MethodKey {
            class: class.name.clone(),
            name: "<init>".to_string(),
            descriptor: MethodType::from_descriptor(&signature).unwrap(),
        }).unwrap();
        constr
    };

    let object_ref = args.runtime.heap.new_object(class.deref());

    let constr_args = if constr.descriptor.parameters.len() > 0 {
        let args_arr = args.runtime.heap.get_array(args_arr_ref);
        let mut constr_args = vec![];
        constr_args.push(Value::Reference(object_ref));
        for idx in 0..args_arr.length().0 {
            constr_args.push(args_arr.get_element(Int(idx)));
        }
        constr_args
    } else {
        vec![Value::Reference(object_ref)]
    };

    let thread = unsafe { args.thread.cast_mut().as_mut().unwrap() };

    let (_, ex) = thread.native_invoke(class.deref() as *const ObjectClass, constr as *const method_area::Method, constr_args);
    if ex.is_some() {
        (None, ex)
    } else {
        (Some(Value::Reference(object_ref)), None)
    }
}

fn register_natives(_: &Args) -> (Option<Value>, Option<Value>) {
    (None, None)
}

fn identity_hash_code(args: &Args) -> (Option<Value>, Option<Value>) {
    let object_ref = args.params[0].reference();

    if object_ref.0 == 0 {
        return (Some(Value::Int(Int(0))), None);
    }

    let object = args.runtime.heap.get_object(object_ref);
    let hash_code = object.header().hash_code;
    (Some(Value::Int(hash_code)), None)
}

fn init_properties(args: &Args) -> (Option<Value>, Option<Value>) {
    // We need to insert some normal properties now!
    let initial_props = hashmap! {
        "file.encoding" => "UTF-8",
        "file.separator" => "/",
        "line.separator" => "\n",
        "path.separator" => ":",
        "java.home" => "/Users/kitch/Code/robusta/",
        "java.library.path" => "/Users/kitch/Code/robusta/target/debug",
        "sun.boot.library.path" => "/Users/kitch/Code/robusta/target/debug"
    };

    let props = args.params[0].reference();
    let properties_class = args.runtime.method_area.load_class("java.util.Properties");
    let properties_set = properties_class.find_method(&MethodKey {
        class: "java.util.Properties".to_string(),
        name: "setProperty".to_string(),
        descriptor: MethodType::from_descriptor("(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;").unwrap(),
    }).unwrap();

    let thread = unsafe { args.thread.cast_mut().as_mut().unwrap() };

    for (key, val) in initial_props {
        let key = args.runtime.method_area.load_string(key);
        let val = args.runtime.method_area.load_string(val);
        let (_, ex) = thread.native_invoke(properties_class.deref() as *const ObjectClass, properties_set as *const method_area::Method, vec![
            Value::Reference(props),
            Value::Reference(key),
            Value::Reference(val),
        ]);
        if ex.is_some() {
            return (None, ex);
        }
    }
    (Some(Value::Reference(Reference(0))), None)
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

fn map_library_name(args: &Args) -> (Option<Value>, Option<Value>) {
    let name = args.params[0].reference();
    let name = args.runtime.heap.get_string(name);

    let libname = format!("librobusta_{}.dylib", name);
    let libname = args.runtime.method_area.load_string(&libname);

    (Some(Value::Reference(libname)), None)
}

fn find_builtin(_: &Args) -> (Option<Value>, Option<Value>) {
    // let name = args.params[0].reference();
    // let name = args.runtime.heap.get_string(name);
    // (Some(Value::Reference(name)), None)
    (Some(Value::Reference(Reference(0))), None)
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
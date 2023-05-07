use std::collections::HashMap;
use std::mem::size_of;
use std::ops::Deref;
use std::path::PathBuf;
use std::ptr;
use std::sync::Arc;
use std::thread::{available_parallelism, Builder, current, sleep};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use nohash_hasher::BuildNoHashHasher;

use rand::{RngCore, thread_rng};

use crate::class_file::Code;
use crate::collection::once::Once;
use crate::collection::wait::ThreadWait;
use crate::heap::allocator::ArrayHeader;
use crate::java::{Double, FieldType, Int, Long, MethodType, Reference, Value};
use crate::method_area;
use crate::method_area::{Class, ClassFlags, ObjectClass};
use crate::method_area::const_pool::{ClassKey, Const, ConstPool, FieldKey, MethodKey, SymbolicReference};
use crate::native::{Args, Plugin};
use crate::native::stateless::{Method, stateless};
use crate::thread::Thread;

pub fn java_lang_plugins() -> Vec<Arc<dyn Plugin>> {
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
                name: "registerNatives".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(no_op),
        ),
        stateless(
            Method {
                class: "java.io.UnixFileSystem".to_string(),
                name: "initIDs".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(no_op),
        ),
        stateless(
            Method {
                class: "java.io.UnixFileSystem".to_string(),
                name: "canonicalize0".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/String;)Ljava/lang/String;").unwrap(),
            },
            Arc::new(unix_canonicalize),
        ),
        stateless(
            Method {
                class: "java.lang.System".to_string(),
                name: "currentTimeMillis".to_string(),
                descriptor: MethodType::from_descriptor("()J").unwrap(),
            },
            Arc::new(current_time_millis),
        ),
        stateless(
            Method {
                class: "java.lang.Object".to_string(),
                name: "notifyAll".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(no_op),
        ),
        stateless(
            Method {
                class: "java.lang.Runtime".to_string(),
                name: "availableProcessors".to_string(),
                descriptor: MethodType::from_descriptor("()I").unwrap(),
            },
            Arc::new(available_processors),
        ),
        stateless(
            Method {
                class: "sun.misc.VM".to_string(),
                name: "initialize".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(no_op),
        ),
        stateless(
            Method {
                class: "sun.misc.Unsafe".to_string(),
                name: "arrayBaseOffset".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/Class;)I").unwrap(),
            },
            Arc::new(array_base_offset),
        ),
        stateless(
            Method {
                class: "sun.misc.Unsafe".to_string(),
                name: "arrayIndexScale".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/Class;)I").unwrap(),
            },
            Arc::new(array_index_scale),
        ),
        stateless(
            Method {
                class: "sun.misc.Unsafe".to_string(),
                name: "addressSize".to_string(),
                descriptor: MethodType::from_descriptor("()I").unwrap(),
            },
            Arc::new(address_size),
        ),
        stateless(
            Method {
                class: "sun.reflect.Reflection".to_string(),
                name: "getCallerClass".to_string(),
                descriptor: MethodType::from_descriptor("()Ljava/lang/Class;").unwrap(),
            },
            Arc::new(get_caller_class),
        ),
        stateless(
            Method {
                class: "java.lang.Class".to_string(),
                name: "isPrimitive".to_string(),
                descriptor: MethodType::from_descriptor("()Z").unwrap(),
            },
            Arc::new(is_primitive),
        ),
        stateless(
            Method {
                class: "java.lang.Class".to_string(),
                name: "isInterface".to_string(),
                descriptor: MethodType::from_descriptor("()Z").unwrap(),
            },
            Arc::new(is_interface),
        ),
        stateless(
            Method {
                class: "java.io.FileInputStream".to_string(),
                name: "initIDs".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(no_op),
        ),
        stateless(
            Method {
                class: "java.io.FileOutputStream".to_string(),
                name: "initIDs".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(no_op),
        ),
        stateless(
            Method {
                class: "java.io.FileDescriptor".to_string(),
                name: "initIDs".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(no_op),
        ),
        stateless(
            Method {
                class: "java.lang.Class".to_string(),
                name: "registerNatives".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(no_op),
        ),
        stateless(
            Method {
                class: "java.lang.Class".to_string(),
                name: "desiredAssertionStatus0".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/Class;)Z").unwrap(),
            },
            Arc::new(assertion_status),
        ),
        stateless(
            Method {
                class: "java.lang.Float".to_string(),
                name: "floatToRawIntBits".to_string(),
                descriptor: MethodType::from_descriptor("(F)I").unwrap(),
            },
            Arc::new(float_to_int_bits),
        ),
        stateless(
            Method {
                class: "java.lang.Double".to_string(),
                name: "doubleToRawLongBits".to_string(),
                descriptor: MethodType::from_descriptor("(D)J").unwrap(),
            },
            Arc::new(double_to_long_bits),
        ),
        stateless(
            Method {
                class: "java.lang.Double".to_string(),
                name: "longBitsToDouble".to_string(),
                descriptor: MethodType::from_descriptor("(J)D").unwrap(),
            },
            Arc::new(long_bits_to_double),
        ),
        stateless(
            Method {
                class: "java.lang.Class".to_string(),
                name: "getPrimitiveClass".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/String;)Ljava/lang/Class;").unwrap(),
            },
            Arc::new(get_primitive_class),
        ),
        stateless(
            Method {
                class: "java.lang.Class".to_string(),
                name: "isArray".to_string(),
                descriptor: MethodType::from_descriptor("()Z").unwrap(),
            },
            Arc::new(class_is_array),
        ),
        stateless(
            Method {
                class: "java.lang.Class".to_string(),
                name: "getComponentType".to_string(),
                descriptor: MethodType::from_descriptor("()Ljava/lang/Class;").unwrap(),
            },
            Arc::new(class_get_component_type),
        ),
        stateless(
            Method {
                class: "java.lang.ClassLoader".to_string(),
                name: "registerNatives".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(no_op),
        ),
        stateless(
            Method {
                class: "java.lang.Object".to_string(),
                name: "getClass".to_string(),
                descriptor: MethodType::from_descriptor("()Ljava/lang/Class;").unwrap(),
            },
            Arc::new(object_get_class),
        ),
        stateless(
            Method {
                class: "java.lang.reflect.Array".to_string(),
                name: "newArray".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/Class;I)Ljava/lang/Object;").unwrap(),
            },
            Arc::new(array_new_array),
        ),
        stateless(
            Method {
                class: "java.lang.Object".to_string(),
                name: "clone".to_string(),
                descriptor: MethodType::from_descriptor("()Ljava/lang/Object;").unwrap(),
            },
            Arc::new(object_clone),
        ),
        stateless(
            Method {
                class: "java.lang.Object".to_string(),
                name: "wait".to_string(),
                descriptor: MethodType::from_descriptor("(J)V").unwrap(),
            },
            Arc::new(object_wait),
        ),
        stateless(
            Method {
                class: "java.lang.Object".to_string(),
                name: "hashCode".to_string(),
                descriptor: MethodType::from_descriptor("()I").unwrap(),
            },
            Arc::new(object_hash_code),
        ),
        stateless(
            Method {
                class: "java.lang.Throwable".to_string(),
                name: "fillInStackTrace".to_string(),
                descriptor: MethodType::from_descriptor("(I)Ljava/lang/Throwable;").unwrap(),
            },
            Arc::new(fill_in_stack_trace),
        ),
        stateless(
            Method {
                class: "java.lang.Throwable".to_string(),
                name: "getStackTraceDepth".to_string(),
                descriptor: MethodType::from_descriptor("()I").unwrap(),
            },
            Arc::new(stack_trace_depth),
        ),
        stateless(
            Method {
                class: "java.lang.Throwable".to_string(),
                name: "getStackTraceElement".to_string(),
                descriptor: MethodType::from_descriptor("(I)Ljava/lang/StackTraceElement;").unwrap(),
            },
            Arc::new(stack_trace_elem),
        ),
        stateless(
            Method {
                class: "java.lang.Class".to_string(),
                name: "forName0".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;").unwrap(),
            },
            Arc::new(for_name_0),
        ),
        stateless(
            Method {
                class: "java.lang.Integer".to_string(),
                name: "toString".to_string(),
                descriptor: MethodType::from_descriptor("(I)Ljava/lang/String;").unwrap(),
            },
            Arc::new(integer_to_string),
        ),
        stateless(
            Method {
                class: "java.lang.Class".to_string(),
                name: "getDeclaredFields0".to_string(),
                descriptor: MethodType::from_descriptor("(Z)[Ljava/lang/reflect/Field;").unwrap(),
            },
            Arc::new(get_declared_fields),
        ),
        stateless(
            Method {
                class: "java.lang.Class".to_string(),
                name: "getDeclaredConstructors0".to_string(),
                descriptor: MethodType::from_descriptor("(Z)[Ljava/lang/reflect/Constructor;").unwrap(),
            },
            Arc::new(get_declared_constructors),
        ),
        stateless(
            Method {
                class: "sun.misc.Unsafe".to_string(),
                name: "objectFieldOffset".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/reflect/Field;)J").unwrap(),
            },
            Arc::new(field_offset),
        ),
        stateless(
            Method {
                class: "sun.misc.Unsafe".to_string(),
                name: "compareAndSwapObject".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z").unwrap(),
            },
            Arc::new(compare_and_swap),
        ),
        stateless(
            Method {
                class: "sun.misc.Unsafe".to_string(),
                name: "compareAndSwapInt".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/Object;JII)Z").unwrap(),
            },
            Arc::new(compare_and_swap_int),
        ),
        stateless(
            Method {
                class: "sun.reflect.Reflection".to_string(),
                name: "getClassAccessFlags".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/Class;)I").unwrap(),
            },
            Arc::new(get_class_access_flags),
        ),
        stateless(
            Method {
                class: "java.lang.Class".to_string(),
                name: "getModifiers".to_string(),
                descriptor: MethodType::from_descriptor("()I").unwrap(),
            },
            Arc::new(get_modifiers),
        ),
        stateless(
            Method {
                class: "java.lang.Thread".to_string(),
                name: "start0".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(thread_start),
        ),
        stateless(
            Method {
                class: "java.lang.Thread".to_string(),
                name: "registerNatives".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(no_op),
        ),
        stateless(
            Method {
                class: "java.lang.Thread".to_string(),
                name: "sleep".to_string(),
                descriptor: MethodType::from_descriptor("(J)V").unwrap(),
            },
            Arc::new(thread_sleep),
        ),
        stateless(
            Method {
                class: "java.lang.Thread".to_string(),
                name: "join".to_string(),
                descriptor: MethodType::from_descriptor("()V").unwrap(),
            },
            Arc::new(thread_join),
        ),
        stateless(
            Method {
                class: "java.lang.Thread".to_string(),
                name: "setPriority0".to_string(),
                descriptor: MethodType::from_descriptor("(I)V").unwrap(),
            },
            Arc::new(set_priority_0),
        ),
        stateless(
            Method {
                class: "java.lang.Thread".to_string(),
                name: "isAlive".to_string(),
                descriptor: MethodType::from_descriptor("()Z").unwrap(),
            },
            Arc::new(thread_is_alive),
        ),
        stateless(
            Method {
                class: "java.lang.Thread".to_string(),
                name: "join".to_string(),
                descriptor: MethodType::from_descriptor("(J)V").unwrap(),
            },
            Arc::new(thread_join_millis),
        ),
        stateless(
            Method {
                class: "java.lang.Thread".to_string(),
                name: "currentThread".to_string(),
                descriptor: MethodType::from_descriptor("()Ljava/lang/Thread;").unwrap(),
            },
            Arc::new(current_thread),
        ),
        stateless(
            Method {
                class: "java.security.AccessController".to_string(),
                name: "getStackAccessControlContext".to_string(),
                descriptor: MethodType::from_descriptor("()Ljava/security/AccessControlContext;").unwrap(),
            },
            Arc::new(get_control_context),
        ),
        stateless(
            Method {
                class: "java.security.AccessController".to_string(),
                name: "getInheritedAccessControlContext".to_string(),
                descriptor: MethodType::from_descriptor("()Ljava/security/AccessControlContext;").unwrap(),
            },
            Arc::new(get_control_context),
        ),
        stateless(
            Method {
                class: "java.lang.Class".to_string(),
                name: "isAssignableFrom".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/Class;)Z").unwrap(),
            },
            Arc::new(is_assignable_from),
        ),
        stateless(
            Method {
                class: "java.lang.Class".to_string(),
                name: "getSuperclass".to_string(),
                descriptor: MethodType::from_descriptor("()Ljava/lang/Class;").unwrap(),
            },
            Arc::new(get_super_class),
        ),
        stateless(
            Method {
                class: "java.lang.System".to_string(),
                name: "arraycopy".to_string(),
                descriptor: MethodType::from_descriptor("(Ljava/lang/Object;ILjava/lang/Object;II)V").unwrap(),
            },
            Arc::new(array_copy),
        ),
    ]
}

fn class_is_array(args: &Args) -> (Option<Value>, Option<Value>) {
    let class = args.params[0].reference();
    let class_object = args.runtime.heap.get_object(class);
    let name = class_object.get_string("name", &args.runtime.heap);

    let is_array = name.starts_with('[');
    let is_array = if is_array { 1 } else { 0 };

    (Some(Value::Int(Int(is_array))), None)
}

fn class_get_component_type(args: &Args) -> (Option<Value>, Option<Value>) {
    let class = args.params[0].reference();
    let class_object = args.runtime.heap.get_object(class);
    let name = class_object.get_string("name", &args.runtime.heap);

    let class_class = args.runtime.method_area.load_outer_class(&name);

    let comp = match class_class {
        Class::Array { component, .. } => component.as_ref().clone(),
        _ => panic!("error")
    };

    let comp_class_instance = args.runtime.method_area.load_class_object(comp);

    (Some(Value::Reference(comp_class_instance)), None)
}

fn available_processors(_: &Args) -> (Option<Value>, Option<Value>) {
    let cores = available_parallelism().unwrap().get();
    (Some(Value::Int(Int(cores as i32))), None)
}

fn unix_canonicalize(args: &Args) -> (Option<Value>, Option<Value>) {
    let path = args.params[1].reference();
    let path = args.runtime.heap.get_string(path);
    let path = PathBuf::from(&path);
    let path = path.canonicalize().unwrap().to_str().unwrap().to_string();
    let path = args.runtime.method_area.load_string(&path);
    (Some(Value::Reference(path)), None)
}

fn current_time_millis(_: &Args) -> (Option<Value>, Option<Value>) {
    let millis = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
    (Some(Value::Long(Long(millis))), None)
}

fn get_declared_fields(args: &Args) -> (Option<Value>, Option<Value>) {
    let thread = unsafe { args.thread.cast_mut().as_mut().unwrap() };

    let class_ref = args.params[0].reference();
    let class_obj = args.runtime.heap.get_object(class_ref);
    let class_name_ref = class_obj.get_field(&FieldKey {
        class: "java.lang.String".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let class_name = args.runtime.heap.get_string(class_name_ref);
    let class = args.runtime.method_area.load_outer_class(&class_name);

    let field_class = args.runtime.method_area.load_outer_class("java.lang.reflect.Field");
    let field_class_obj = field_class.obj();
    let field_init = field_class_obj.find_method(&MethodKey {
        class: "java.lang.reflect.Field".to_string(),
        name: "<init>".to_string(),
        descriptor: MethodType::from_descriptor("(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;IILjava/lang/String;[B)V").unwrap(),
    }).unwrap();

    let mut all_fields = vec![];

    if let Class::Object(class) = class {
        let fields = class.instance_fields.iter()
            .chain(class.static_fields.iter());
        for field in fields {
            let declaring_class = class_ref;
            let name = args.runtime.heap.insert_string_const(
                &field.name,
                args.runtime.method_area.load_class("java.lang.String").deref());
            let field_type = args.runtime.method_area.load_outer_class(&field.descriptor.as_class());
            let field_type = args.runtime.method_area.load_class_object(field_type);
            let modifiers = Int(field.flags as i32);
            let slot = Int(0);
            let signature = Reference(0);
            let annotations = Reference(0);

            let field = args.runtime.heap.new_object(&field_class_obj);

            let (_, ex) = thread.native_invoke(
                field_class_obj.deref() as *const ObjectClass,
                field_init as *const method_area::Method,
                vec![
                    Value::Reference(field),
                    Value::Reference(declaring_class),
                    Value::Reference(name),
                    Value::Reference(field_type),
                    Value::Int(modifiers),
                    Value::Int(slot),
                    Value::Reference(signature),
                    Value::Reference(annotations),
                ],
            );
            if ex.is_some() {
                return (None, ex);
            }
            all_fields.push(field);
        }
    }

    let fields_array_ref = args.runtime.heap.new_array(field_class, Int(all_fields.len() as i32));
    let fields_array = args.runtime.heap.get_array(fields_array_ref);

    for (idx, field) in all_fields.iter().enumerate() {
        fields_array.set_element(Int(idx as i32), Value::Reference(*field));
    }

    (Some(Value::Reference(fields_array_ref)), None)
}

fn get_declared_constructors(args: &Args) -> (Option<Value>, Option<Value>) {
    let thread = unsafe { args.thread.cast_mut().as_mut().unwrap() };

    let class_class = args.runtime.method_area.load_outer_class("java.lang.Class");

    let class_ref = args.params[0].reference();
    let class_obj = args.runtime.heap.get_object(class_ref);
    let class_name_ref = class_obj.get_field(&FieldKey {
        class: "java.lang.String".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let class_name = args.runtime.heap.get_string(class_name_ref);
    let class = args.runtime.method_area.load_outer_class(&class_name);

    let constr_class = args.runtime.method_area.load_outer_class("java.lang.reflect.Constructor");
    let constr_class_obj = constr_class.obj();
    let constr_init = constr_class_obj.find_method(&MethodKey {
        class: "java.lang.reflect.Constructor".to_string(),
        name: "<init>".to_string(),
        descriptor: MethodType::from_descriptor("(Ljava/lang/Class;[Ljava/lang/Class;[Ljava/lang/Class;IILjava/lang/String;[B[B)V").unwrap(),
    }).unwrap();

    let mut all_constructors = vec![];

    if let Class::Object(class) = class {
        for method in &class.methods {
            if method.name.ne("<init>") {
                continue;
            }
            let declaring_class = class_ref;
            let param_classes: Vec<Reference> = method.descriptor.parameters.iter()
                .map(|ft| args.runtime.method_area.load_outer_class(&ft.as_class()))
                .map(|class| args.runtime.method_area.load_class_object(class))
                .collect();
            let params_array_ref = args.runtime.heap.new_array(class_class.clone(), Int(param_classes.len() as i32));
            let params_array = args.runtime.heap.get_array(params_array_ref);
            for (idx, param) in param_classes.iter().enumerate() {
                params_array.set_element(Int(idx as i32), Value::Reference(*param));
            }
            let checked_exceptions = args.runtime.heap.new_array(class_class.clone(), Int(0));

            let modifiers = Int(method.flags as i32);
            let slot = Int(0);
            let signature = Reference(0);
            let annotations = Reference(0);
            let param_annotations = Reference(0);

            let constr = args.runtime.heap.new_object(&constr_class_obj);

            let (_, ex) = thread.native_invoke(
                constr_class_obj.deref() as *const ObjectClass,
                constr_init as *const method_area::Method,
                vec![
                    Value::Reference(constr),
                    Value::Reference(declaring_class),
                    Value::Reference(params_array_ref),
                    Value::Reference(checked_exceptions),
                    Value::Int(modifiers),
                    Value::Int(slot),
                    Value::Reference(signature),
                    Value::Reference(annotations),
                    Value::Reference(param_annotations),
                ],
            );
            if ex.is_some() {
                return (None, ex);
            }
            all_constructors.push(constr);
        }
    }

    let constr_array_ref = args.runtime.heap.new_array(constr_class, Int(all_constructors.len() as i32));
    let constr_array = args.runtime.heap.get_array(constr_array_ref);

    for (idx, constr) in all_constructors.iter().enumerate() {
        constr_array.set_element(Int(idx as i32), Value::Reference(*constr));
    }

    (Some(Value::Reference(constr_array_ref)), None)
}

fn field_offset(args: &Args) -> (Option<Value>, Option<Value>) {
    let field_ref = args.params[1].reference();
    let field_obj = args.runtime.heap.get_object(field_ref);

    let name_ref = field_obj.get_field(&FieldKey {
        class: "java.lang.String".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let name = args.runtime.heap.get_string(name_ref);

    let class_ref = field_obj.get_field(&FieldKey {
        class: "java.lang.reflect.Field".to_string(),
        name: "clazz".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/Class;").unwrap(),
    }).reference();
    let class_obj = args.runtime.heap.get_object(class_ref);
    let class_name_ref = class_obj.get_field(&FieldKey {
        class: "java.lang.Class".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let class_name = args.runtime.heap.get_string(class_name_ref);

    let class = args.runtime.method_area.load_class(&class_name);
    let field = class.instance_fields.iter().find(|f| f.name.eq(&name)).unwrap();
    let offset = field.offset as i64;

    (Some(Value::Long(Long(offset))), None)
}

fn get_class_access_flags(args: &Args) -> (Option<Value>, Option<Value>) {
    let class_ref = args.params[0].reference();
    let class_obj = args.runtime.heap.get_object(class_ref);

    let name_ref = class_obj.get_field(&FieldKey {
        class: "java.lang.Class".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let name = args.runtime.heap.get_string(name_ref);
    let class = args.runtime.method_area.load_class(&name);

    let flags = class.flags.bits as i32;

    (Some(Value::Int(Int(flags))), None)
}

fn get_modifiers(args: &Args) -> (Option<Value>, Option<Value>) {
    let class_ref = args.params[0].reference();
    let class_obj = args.runtime.heap.get_object(class_ref);

    let name_ref = class_obj.get_field(&FieldKey {
        class: "java.lang.Class".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let name = args.runtime.heap.get_string(name_ref);
    let class = args.runtime.method_area.load_class(&name);

    let flags = class.flags.bits as i32;

    (Some(Value::Int(Int(flags))), None)
}

fn compare_and_swap(args: &Args) -> (Option<Value>, Option<Value>) {
    let o = args.params[1].reference();
    let offset = args.params[2].long().0 as usize;
    let expected = args.params[3].reference().0;
    let x = args.params[4].reference().0;

    let object = args.runtime.heap.get_object(o);

    let result = unsafe {
        let ptr: *mut u32 = object.data.add(offset).cast();
        let current = ptr.read_volatile();
        if current == expected {
            ptr.write_volatile(x);
            1
        } else {
            0
        }
    };

    (Some(Value::Int(Int(result))), None)
}

fn compare_and_swap_int(args: &Args) -> (Option<Value>, Option<Value>) {
    let o = args.params[1].reference();
    let offset = args.params[2].long().0 as usize;
    let expected = args.params[3].int().0;
    let x = args.params[4].int().0;

    let object = args.runtime.heap.get_object(o);

    let result = unsafe {
        let ptr: *mut i32 = object.data.add(offset).cast();
        let current = ptr.read_volatile();
        if current == expected {
            ptr.write_volatile(x);
            1
        } else {
            0
        }
    };

    (Some(Value::Int(Int(result))), None)
}

fn integer_to_string(args: &Args) -> (Option<Value>, Option<Value>) {
    let int = args.params[0].int();

    let string_rep = format!("{}", int.0);
    let string_ref = args.runtime.heap.insert_string_const(&string_rep, &*args.runtime.method_area.load_class("java.lang.String"));

    (Some(Value::Reference(string_ref)), None)
}

fn string_intern(args: &Args) -> (Option<Value>, Option<Value>) {
    let string_ref = args.params[0].reference();
    let string_obj = args.runtime.heap.get_object(string_ref);

    let chars_ref = string_obj.get_field(&FieldKey {
        class: "java.lang.String".to_string(),
        name: "value".to_string(),
        descriptor: FieldType::from_descriptor("[C").unwrap(),
    }).reference();

    let chars = args.runtime.heap.get_array(chars_ref);
    let chars = chars.as_chars_slice();

    let string = String::from_utf16(chars).unwrap();
    let string_ref = args.runtime.heap.insert_string_const(&string, string_obj.class());

    (Some(Value::Reference(string_ref)), None)
}

fn object_get_class(args: &Args) -> (Option<Value>, Option<Value>) {
    let object_ref = args.params[0].reference();
    let object_obj = args.runtime.heap.get(object_ref);

    let class_ref = args.runtime.method_area.load_class_object(object_obj.class(
        args.runtime.method_area.load_outer_class("java.lang.Object")
    ));

    (Some(Value::Reference(class_ref)), None)
}

fn array_new_array(args: &Args) -> (Option<Value>, Option<Value>) {
    let component_ref = args.params[0].reference();
    let component_obj = args.runtime.heap.get_object(component_ref);

    let name_ref = component_obj.get_field(&FieldKey {
        class: "java.lang.String".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let name = args.runtime.heap.get_string(name_ref);
    let class = args.runtime.method_area.load_outer_class(&name);

    let length = args.params[1].int();

    let array = args.runtime.heap.new_array(class, length);

    (Some(Value::Reference(array)), None)
}

fn object_clone(args: &Args) -> (Option<Value>, Option<Value>) {
    let object_ref = args.params[0].reference();
    let object_obj = args.runtime.heap.get(object_ref);

    let copied = args.runtime.heap.copy(object_obj);

    (Some(Value::Reference(copied)), None)
}

fn object_wait(args: &Args) -> (Option<Value>, Option<Value>) {

    let thread = unsafe { args.thread.cast_mut().as_mut().unwrap() };

    let object_ref = args.params[0].reference();
    let millis = args.params[1].long();

    let mut sync = thread.locks.remove(&object_ref.0).expect("Do not hold the lock on this object");
    let reentry = sync.reentry;
    drop(sync);

    let object_obj = args.runtime.heap.get_object(object_ref);

    let lock = &object_obj.header().lock;

    // Entering safe region!
    thread.safe.enter();

    let timeout = if millis.0 == 0 { None } else { Some(Duration::from_millis(millis.0 as u64)) };

    lock.wait(timeout);

    let mut sync = lock.lock();
    sync.reentry = reentry;
    thread.locks.insert(object_ref.0, sync);

    thread.safe.exit();

    (None, None)
}

fn object_hash_code(args: &Args) -> (Option<Value>, Option<Value>) {
    let object_ref = args.params[0].reference();
    let object_obj = args.runtime.heap.get_object(object_ref);

    let hash_code = object_obj.hash_code();

    (Some(Value::Int(hash_code)), None)
}

fn fill_in_stack_trace(args: &Args) -> (Option<Value>, Option<Value>) {
    let throwable_class = args.runtime.method_area.load_class("java.lang.Throwable");

    let throwable_ref = args.params[0].reference();

    let thread = unsafe { args.thread.cast_mut().as_mut().unwrap() };

    let stack = thread.stack.iter().rev()
        .filter(|f| !f.class.starts_with('<'))
        .filter(|f| {
            !(f.class.eq("com.jkitch.robusta.Robusta") &&
                unsafe { f.method.as_ref().unwrap() }.name.eq("throwThrowable"))
        })
        .skip_while(|f| {
            let method = unsafe { f.method.as_ref().unwrap() };
            let class = unsafe { method.class.as_ref().unwrap() };
            class.is_instance_of(&throwable_class) &&
                (method.name.eq("<init>") || method.name.eq("fillInStackTrace"))
        });

    let mut elems: Vec<StackElem> = stack.map(|frame| {
        let method = unsafe { frame.method.as_ref().unwrap() };
        let class = unsafe { method.class.as_ref().unwrap() };
        StackElem {
            class: frame.class.clone(),
            method: method.name.clone(),
            file: class.source_file.clone(),
            line: {
                let line_numbers = method.code.as_ref().and_then(|code| code.line_number_table())
                    .map(|table| &table.table);
                if let Some(table) = line_numbers {
                    table.iter()
                        .filter(|ln| ln.start_pc as usize <= frame.pc)
                        .last()
                        .map(|ln| ln.line_number)
                        .unwrap() as i32
                } else {
                    -2
                }
            },
        }
    }).collect();

    elems.reverse();

    // Can we create a class that delegates to all our methods for us?
    let mut class = ObjectClass {
        name: format!("<fill-in-stack-trace-{:?}-{}>", current().id(), thread_rng().next_u64()),
        flags: ClassFlags { bits: 0 },
        const_pool: ConstPool {
            pool: HashMap::with_hasher(BuildNoHashHasher::default()),
        },
        super_class: None,
        interfaces: vec![],
        instance_fields: vec![],
        static_fields: vec![],
        methods: vec![],
        attributes: vec![],
        instance_width: 0,
        static_width: 0,
        source_file: None,
    };

    let mut method = method_area::Method {
        class: 0 as *const ObjectClass, // TODO: Fill in later!
        is_static: true,
        is_native: false,
        is_synchronized: false,
        flags: 0,
        name: "<fill-in-stack-trace>".to_string(),
        descriptor: MethodType::from_descriptor("()[Ljava/lang/StackTraceElement;").unwrap(),
        code: Some(Code {
            max_stack: 0,
            max_locals: 0,
            code: vec![],
            ex_table: vec![],
            attributes: vec![],
        }),
    };

    let code = &mut method.code.as_mut().unwrap().code;

    let mut idx = 1;

    // Class Const
    class.const_pool.pool.insert(idx, Const::Class(SymbolicReference {
        const_key: ClassKey { name: "java.lang.StackTraceElement".to_string() },
        resolved: Once::new(),
    }));
    let class_const_idx = idx;
    idx += 1;

    // Constructor const
    class.const_pool.pool.insert(idx, Const::Method(SymbolicReference {
        const_key: MethodKey {
            class: "java.lang.StackTraceElement".to_string(),
            name: "<init>".to_string(),
            descriptor: MethodType::from_descriptor("(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;I)V").unwrap(),
        },
        resolved: Once::new(),
    }));
    let method_const_idx = idx;
    idx += 1;

    // Length of array const
    class.const_pool.pool.insert(idx, Const::Integer(elems.len() as i32));
    let array_len_const = idx;
    idx += 1;

    // Insert array length constant onto stack
    code.push(0x13); // ldc_w
    code.push(array_len_const.to_be_bytes()[0]);
    code.push(array_len_const.to_be_bytes()[1]);

    // Create new array
    code.push(0xBD); // anewarray
    code.push(class_const_idx.to_be_bytes()[0]);
    code.push(class_const_idx.to_be_bytes()[1]);

    // Store array into local var 0
    code.push(0x4b); // astore_0 -> array into local var 0

    for elem in &elems {
        // Create new element
        code.push(0xBB); // NEW
        code.push(class_const_idx.to_be_bytes()[0]);
        code.push(class_const_idx.to_be_bytes()[1]);

        // Duplicate object ref for later.
        code.push(0x59);

        // Class const & load const.
        class.const_pool.pool.insert(idx, Const::String(SymbolicReference {
            const_key: elem.class.clone(),
            resolved: Once::new(),
        }));
        code.push(0x13); // ldc_w
        code.push(idx.to_be_bytes()[0]);
        code.push(idx.to_be_bytes()[1]);
        idx += 1;

        // Method const & load const
        class.const_pool.pool.insert(idx, Const::String(SymbolicReference {
            const_key: elem.method.clone(),
            resolved: Once::new(),
        }));
        code.push(0x13); // ldc_w
        code.push(idx.to_be_bytes()[0]);
        code.push(idx.to_be_bytes()[1]);
        idx += 1;

        // File const
        if let Some(file) = &elem.file {
            class.const_pool.pool.insert(idx, Const::String(SymbolicReference {
                const_key: file.clone(),
                resolved: Once::new(),
            }));
            code.push(0x13); // ldc_w
            code.push(idx.to_be_bytes()[0]);
            code.push(idx.to_be_bytes()[1]);
            idx += 1;
        } else {
            code.push(0x1); // aconst_null
        }

        // Line Number const & load
        class.const_pool.pool.insert(idx, Const::Integer(elem.line));
        code.push(0x13); // ldc_w
        code.push(idx.to_be_bytes()[0]);
        code.push(idx.to_be_bytes()[1]);
        idx += 1;

        // Invoke constructor
        code.push(0xB7); // invokespecial
        code.push(method_const_idx.to_be_bytes()[0]);
        code.push(method_const_idx.to_be_bytes()[1]);
    }

    // Store index in local_var 1
    code.push(0x3); // 0 const
    code.push(0x3C); // istore_1

    for _ in 0..elems.len() {
        // Pop element off stack into local var 2
        code.push(0x4d);

        // Load array ref from local var 0
        code.push(0x2a);

        // Load index from local var 1
        code.push(0x1b);

        // Load element from local var 2
        code.push(0x2c);

        // Store in array
        code.push(0x53);

        // Increment index in local var 1 by 1
        code.push(0x84);
        code.push(1);
        code.push(1);
    }

    // Load array ref from local var 0
    code.push(0x2a);

    // Return array ref
    code.push(0xb0);

    class.methods.push(method);

    let class = args.runtime.method_area.insert_gen_class(class);
    let method = &unsafe { class.as_ref().unwrap() }.methods[0] as *const method_area::Method;

    let (array_reference, ex) = thread.native_invoke(class, method, vec![]);
    if ex.is_some() {
        return (None, ex);
    }

    // Store array reference in field
    let throwable = args.runtime.heap.get_object(throwable_ref);
    throwable.set_field(&FieldKey {
        class: "java.lang.Throwable".to_string(),
        name: "backtrace".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/Object;").unwrap(),
    }, Value::Reference(array_reference.unwrap().reference()));

    (Some(Value::Reference(throwable_ref)), None)
}

struct StackElem {
    class: String,
    method: String,
    file: Option<String>,
    line: i32,
}

fn thread_start(args: &Args) -> (Option<Value>, Option<Value>) {
    let thread_ref = args.params[0].reference();
    let thread_obj = args.runtime.heap.get_object(thread_ref);

    let name_ref = thread_obj.get_field(&FieldKey {
        class: "java.lang.Thread".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let name = args.runtime.heap.get_string(name_ref);

    if name.eq("Reference Handler") {
        // TODO: Handle this thread!
        return (None, None);
    }

    let runtime = args.runtime.clone();
    let class = thread_obj.class().name.clone();

    runtime.threads.insert(name.clone(), ThreadWait::new(runtime.clone(), thread_ref));

    Builder::new().name(name.clone()).spawn(move || {
        let const_pool = &thread_obj.class().const_pool as *const ConstPool;
        let method = thread_obj.class().find_method(&MethodKey {
            class: class.clone(),
            name: "run".to_string(),
            descriptor: MethodType::from_descriptor("()V").unwrap(),
        }).unwrap() as *const method_area::Method;

        let thread = Thread::new(name, Some(thread_ref.clone()), runtime, class, const_pool, method, vec![
            Value::Reference(thread_ref)
        ]);

        // hack
        unsafe {
            let t = thread.as_ref() as *const Thread;
            let t = t as *mut Thread;
            let t = t.as_mut().unwrap();
            t.run();
        }
    }).unwrap();

    (None, None)
}

pub fn thread_sleep(args: &Args) -> (Option<Value>, Option<Value>) {
    let millis = args.params[0].long().0;
    args.enter_safe();
    sleep(Duration::from_millis(millis as u64));
    args.exit_safe();
    (None, None)
}

pub fn thread_join(args: &Args) -> (Option<Value>, Option<Value>) {
    let thread_ref = args.params[0].reference();
    let thread_obj = args.runtime.heap.get_object(thread_ref);

    let name_ref = thread_obj.get_field(&FieldKey {
        class: "java.lang.Thread".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let name = args.runtime.heap.get_string(name_ref);

    args.enter_safe();
    args.runtime.threads.get(&name).unwrap().join();
    args.exit_safe();

    (None, None)
}

pub fn thread_join_millis(args: &Args) -> (Option<Value>, Option<Value>) {
    let thread_ref = args.params[0].reference();
    let thread_obj = args.runtime.heap.get_object(thread_ref);

    let millis = args.params[1].long();

    let name_ref = thread_obj.get_field(&FieldKey {
        class: "java.lang.Thread".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let name = args.runtime.heap.get_string(name_ref);

    args.enter_safe();
    args.runtime.threads.get(&name).unwrap().join_millis(millis.0);
    args.exit_safe();

    (None, None)
}

pub fn current_thread(args: &Args) -> (Option<Value>, Option<Value>) {
    let thread = unsafe { args.thread.cast_mut().as_mut().unwrap() };

    let is_main_thread = current().name().unwrap().eq("main");
    if is_main_thread && thread.reference.is_none() {
        // TODO: We are called in Thread.<clinit>
        // We need to create a main thread instance ourselves directly!

        // Create main thread group & thread.
        let thread_group_class = args.runtime.method_area.load_outer_class("java.lang.ThreadGroup");
        let thread_group_class = thread_group_class.obj();
        let thread_group_init_system = thread_group_class.find_method(&MethodKey {
            class: "java.lang.ThreadGroup".to_string(),
            name: "<init>".to_string(),
            descriptor: MethodType::from_descriptor("()V").unwrap(),
        }).unwrap();
        let thread_group_init_main = thread_group_class.find_method(&MethodKey {
            class: "java.lang.ThreadGroup".to_string(),
            name: "<init>".to_string(),
            descriptor: MethodType::from_descriptor("(Ljava/lang/Void;Ljava/lang/ThreadGroup;Ljava/lang/String;)V").unwrap(),
        }).unwrap();

        let system_thread_group = args.runtime.heap.new_object(&thread_group_class);
        let main_thread_group = args.runtime.heap.new_object(&thread_group_class);
        let main_string = args.runtime.heap.insert_string_const(
            "main",
            &args.runtime.method_area.load_class("java.lang.String"));

        // Init System Thread Group
        let (_, ex) = thread.native_invoke(
            thread_group_class.deref() as *const ObjectClass,
            thread_group_init_system as *const method_area::Method,
            vec![Value::Reference(system_thread_group)]);
        if ex.is_some() {
            return (None, ex);
        }

        // Init Main Thread Group
        let (_, ex) = thread.native_invoke(
            thread_group_class.deref() as *const ObjectClass,
            thread_group_init_main as *const method_area::Method,
            vec![
                Value::Reference(main_thread_group),
                Value::Reference(Reference(0)),
                Value::Reference(system_thread_group),
                Value::Reference(main_string),
            ],
        );
        if ex.is_some() {
            return (None, ex);
        }

        // Create our main thread.
        let thread_class = args.runtime.method_area.load_class("java.lang.Thread");
        let main_thread_ref = args.runtime.heap.new_object(&thread_class);
        let main_thread_obj = args.runtime.heap.get_object(main_thread_ref);

        let main_string = args.runtime.heap.insert_string_const(
            "main",
            args.runtime.method_area.load_class("java.lang.String").deref(),
        );

        // Set the values that we require for the parent.
        main_thread_obj.set_field(&FieldKey {
            class: "java.lang.Thread".to_string(),
            name: "priority".to_string(),
            descriptor: FieldType::Int,
        }, Value::Int(Int(5)));

        main_thread_obj.set_field(&FieldKey {
            class: "java.lang.Thread".to_string(),
            name: "name".to_string(),
            descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
        }, Value::Reference(main_string));

        main_thread_obj.set_field(&FieldKey {
            class: "java.lang.Thread".to_string(),
            name: "group".to_string(),
            descriptor: FieldType::from_descriptor("Ljava/lang/ThreadGroup;").unwrap(),
        }, Value::Reference(main_thread_group));

        thread.reference = Some(main_thread_ref);

        (Some(Value::Reference(main_thread_ref)), None)
    } else {
        // Get the thread ref from the thread.
        let thread_ref = thread.reference.unwrap();
        (Some(Value::Reference(thread_ref)), None)
    }
}

pub fn no_op(_: &Args) -> (Option<Value>, Option<Value>) {
    (None, None)
}

pub fn get_primitive_class(args: &Args) -> (Option<Value>, Option<Value>) {
    let string_ref = args.params[0].reference();
    let primitive = args.runtime.heap.get_string(string_ref);

    let primitive_class = args.runtime.method_area.load_outer_class(&primitive);
    let primitive_object = args.runtime.method_area.load_class_object(primitive_class);

    (Some(Value::Reference(primitive_object)), None)
}

fn assertion_status(_: &Args) -> (Option<Value>, Option<Value>) {
    (Some(Value::Int(Int(0))), None)
}

fn float_to_int_bits(args: &Args) -> (Option<Value>, Option<Value>) {
    let float = args.params[0].float().0;
    let bytes = float.to_be_bytes();
    let int = i32::from_be_bytes(bytes);
    (Some(Value::Int(Int(int))), None)
}

fn double_to_long_bits(args: &Args) -> (Option<Value>, Option<Value>) {
    let double = args.params[0].double().0;
    let bytes = double.to_be_bytes();
    let long = i64::from_be_bytes(bytes);
    (Some(Value::Long(Long(long))), None)
}

fn long_bits_to_double(args: &Args) -> (Option<Value>, Option<Value>) {
    let long = args.params[0].long().0;
    let bytes = long.to_be_bytes();
    let double = f64::from_be_bytes(bytes);
    (Some(Value::Double(Double(double))), None)
}

fn array_base_offset(_: &Args) -> (Option<Value>, Option<Value>) {
    let offset = size_of::<ArrayHeader>() as i32;
    (Some(Value::Int(Int(offset))), None)
}

fn array_index_scale(args: &Args) -> (Option<Value>, Option<Value>) {
    let class_ref = args.params[1].reference();
    let class_obj = args.runtime.heap.get_object(class_ref);

    let name_ref = class_obj.get_field(&FieldKey {
        class: "java.lang.Class".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let name = args.runtime.heap.get_string(name_ref);

    let scale = match name.as_str() {
        "[Z" | "[B" => 1,
        "[C" | "[S" => 2,
        "[J" | "[D" => 8,
        _ => 4,
    };

    (Some(Value::Int(Int(scale))), None)
}

fn address_size(_: &Args) -> (Option<Value>, Option<Value>) {
    (Some(Value::Int(Int(size_of::<*const u8>() as i32))), None)
}

fn get_caller_class(args: &Args) -> (Option<Value>, Option<Value>) {
    let thread = unsafe { args.thread.as_ref().unwrap() };
    let class_name = thread.stack.iter().rev()
        .skip(2) // skip this frame
        .skip_while(|f| f.class.starts_with('<')) // skip internal frames
        .next()
        .map(|f| &f.class)
        .unwrap();

    let class = args.runtime.method_area.load_outer_class(class_name);
    let class_ref = args.runtime.method_area.load_class_object(class);

    (Some(Value::Reference(class_ref)), None)
}

fn for_name_0(args: &Args) -> (Option<Value>, Option<Value>) {
    let thread = unsafe { args.thread.cast_mut().as_mut().unwrap() };

    let name_ref = args.params[0].reference();
    let name = args.runtime.heap.get_string(name_ref);
    let initialize = args.params[1].int().0 != 0;

    let class = args.runtime.method_area.load_outer_class(&name);
    if initialize {
        args.runtime.method_area.initialize(thread, class.obj().deref());
    }

    let class_obj = args.runtime.method_area.load_class_object(class);
    (Some(Value::Reference(class_obj)), None)
}

fn get_control_context(args: &Args) -> (Option<Value>, Option<Value>) {
    let thread = unsafe { args.thread.cast_mut().as_mut().unwrap() };
    let acc_class = args.runtime.method_area.load_outer_class("java.security.AccessControlContext");
    let acc_class = acc_class.obj();
    let acc_init = acc_class.find_method(&MethodKey {
        class: "java.security.AccessControlContext".to_string(),
        name: "<init>".to_string(),
        descriptor: MethodType::from_descriptor("([Ljava/security/ProtectionDomain;)V").unwrap(),
    }).unwrap();

    let pro_dom_class = args.runtime.method_area.load_outer_class("java.security.ProtectionDomain");

    let acc_ref = args.runtime.heap.new_object(&acc_class);
    let domains_ref = args.runtime.heap.new_array(pro_dom_class, Int(0));

    let (_, ex) = thread.native_invoke(
        acc_class.deref() as *const ObjectClass,
        acc_init as *const method_area::Method,
        vec![
            Value::Reference(acc_ref),
            Value::Reference(domains_ref),
        ],
    );
    if ex.is_some() {
        return (None, ex);
    }

    (Some(Value::Reference(acc_ref)), None)
}

fn set_priority_0(_: &Args) -> (Option<Value>, Option<Value>) {
    (None, None)
}

fn thread_is_alive(args: &Args) -> (Option<Value>, Option<Value>) {
    let thread_ref = args.params[0].reference();
    let thread_obj = args.runtime.heap.get_object(thread_ref);

    let thread_status = thread_obj.get_field(&FieldKey {
        class: "java.lang.Thread".to_string(),
        name: "threadStatus".to_string(),
        descriptor: FieldType::Int,
    }).int();

    let is_alive = if thread_status.0 != 0 { 1 } else { 0 };

    (Some(Value::Int(Int(is_alive))), None)
}

fn stack_trace_depth(args: &Args) -> (Option<Value>, Option<Value>) {
    let throw_ref = args.params[0].reference();
    let throw_obj = args.runtime.heap.get_object(throw_ref);

    let backtrace_ref = throw_obj.get_field(&FieldKey {
        class: "java.lang.Throwable".to_string(),
        name: "backtrace".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/Object;").unwrap(),
    }).reference();

    let backtrace_arr = args.runtime.heap.get_array(backtrace_ref);

    (Some(Value::Int(backtrace_arr.length())), None)
}


fn stack_trace_elem(args: &Args) -> (Option<Value>, Option<Value>) {
    let throw_ref = args.params[0].reference();
    let throw_obj = args.runtime.heap.get_object(throw_ref);

    let index = args.params[1].int();

    let backtrace_ref = throw_obj.get_field(&FieldKey {
        class: "java.lang.Throwable".to_string(),
        name: "backtrace".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/Object;").unwrap(),
    }).reference();

    let backtrace_arr = args.runtime.heap.get_array(backtrace_ref);
    let stack_elem = backtrace_arr.get_element(index);

    (Some(stack_elem), None)
}

fn is_assignable_from(args: &Args) -> (Option<Value>, Option<Value>) {
    let class_ref = args.params[0].reference();
    let class_inst = args.runtime.heap.get_object(class_ref);

    let name_ref = class_inst.get_field(&FieldKey {
        class: "java.lang.Class".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let name = args.runtime.heap.get_string(name_ref);

    let other_inst = args.runtime.heap.get_object(args.params[1].reference());
    let other_name_ref = other_inst.get_field(&FieldKey {
        class: "java.lang.Class".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let other_name = args.runtime.heap.get_string(other_name_ref);

    let this_class = args.runtime.method_area.load_outer_class(&name);
    let other_class = args.runtime.method_area.load_outer_class(&other_name);

    let is_assignable = other_class.is_instance_of(&this_class);
    let is_assignable = if is_assignable { 1 } else { 0 };

    (Some(Value::Int(Int(is_assignable))), None)
}

fn is_primitive(args: &Args) -> (Option<Value>, Option<Value>) {
    let class_ref = args.params[0].reference();
    let class_inst = args.runtime.heap.get_object(class_ref);

    let name_ref = class_inst.get_field(&FieldKey {
        class: "java.lang.Class".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let name = args.runtime.heap.get_string(name_ref);

    let is_primitive = match name.as_str() {
        "boolean" | "byte" | "char" | "short" | "int" | "float" | "long" | "double" => true,
        _ => false,
    };
    let is_primitive = if is_primitive { 1 } else { 0 };

    (Some(Value::Int(Int(is_primitive))), None)
}

fn is_interface(args: &Args) -> (Option<Value>, Option<Value>) {
    let class_ref = args.params[0].reference();
    let class_inst = args.runtime.heap.get_object(class_ref);

    let name_ref = class_inst.get_field(&FieldKey {
        class: "java.lang.Class".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let name = args.runtime.heap.get_string(name_ref);

    let class = args.runtime.method_area.load_outer_class(&name);
    let is_interface = class.is_interface();
    let is_interface = if is_interface { 1 } else { 0 };

    (Some(Value::Int(Int(is_interface))), None)
}

fn get_super_class(args: &Args) -> (Option<Value>, Option<Value>) {
    let class_ref = args.params[0].reference();
    let class_inst = args.runtime.heap.get_object(class_ref);

    let name_ref = class_inst.get_field(&FieldKey {
        class: "java.lang.Class".to_string(),
        name: "name".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/lang/String;").unwrap(),
    }).reference();
    let name = args.runtime.heap.get_string(name_ref);

    let this_class = args.runtime.method_area.load_outer_class(&name);

    let parent_ref = match this_class {
        Class::Primitive(_) => Reference(0),
        Class::Array { .. } => {
            let class = args.runtime.method_area.load_outer_class("java.lang.Object");
            args.runtime.method_area.load_class_object(class)
        }
        Class::Object(obj) => {
            if let Some(parent) = &obj.super_class {
                let parent_class = args.runtime.method_area.load_outer_class(&parent.name);
                args.runtime.method_area.load_class_object(parent_class)
            } else {
                Reference(0)
            }
        }
    };

    (Some(Value::Reference(parent_ref)), None)
}

fn array_copy(args: &Args) -> (Option<Value>, Option<Value>) {
    let src = args.params[0].reference();
    let src_pos = args.params[1].int().0;
    let dest = args.params[2].reference();
    let dest_pos = args.params[3].int().0;
    let length = args.params[4].int().0;

    let src_array = args.runtime.heap.get_array(src);
    let dest_array = args.runtime.heap.get_array(dest);

    let src_comp = unsafe { &src_array.header.as_ref().unwrap().component };
    let dest_comp = unsafe { &dest_array.header.as_ref().unwrap().component };

    if !src_comp.is_instance_of(dest_comp) {
        // TODO: We're ignoring these for now :/
        // panic!("cannot do this!");
    }

    let width = src_comp.component_width();

    unsafe {
        let start_offset = (src_pos as usize) * width;
        let src_start = src_array.data.add(start_offset);

        let dest_offset = (dest_pos as usize) * width;
        let dest_start = dest_array.data.add(dest_offset);

        let bytes = (length as usize) * width;

        ptr::copy(src_start.cast_const(), dest_start, bytes);
    }

    (None, None)
}
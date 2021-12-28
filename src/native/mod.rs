use std::ops::Deref;

use crate::descriptor::MethodDescriptor;
use crate::heap::{Array, Ref, Value};
use crate::thread::local_vars::LocalVars;
use crate::thread::Thread;

pub struct NativeMethods {
    classes: Vec<NativeClass>,
}

impl NativeMethods {
    pub fn load() -> Self {
        NativeMethods {
            classes: vec![
                NativeClass {
                    name: String::from("Robusta"),
                    methods: vec![
                        NativeMethod {
                            name: String::from("println"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/String;)V"),
                            function: robusta_println_string,
                        }
                    ],
                }
            ]
        }
    }

    pub fn find_method(&self, class: &str, name: &str, descriptor: &MethodDescriptor) -> NativeFunction {
        self.classes.iter()
            .find(|c| c.name.eq(class))
            .unwrap()
            .methods.iter()
            .find(|m| m.name.eq(name) && m.descriptor.eq(descriptor))
            .unwrap()
            .function
    }
}

struct NativeClass {
    name: String,
    methods: Vec<NativeMethod>,
}

struct NativeMethod {
    name: String,
    descriptor: MethodDescriptor,
    function: NativeFunction,
}

type NativeFunction = fn(thread: &mut Thread, local_vars: LocalVars) -> Option<Value>;

fn robusta_println_string(thread: &mut Thread, local_vars: LocalVars) -> Option<Value> {
    let string_ref = local_vars.load_ref(0);
    let string_obj = thread.object(string_ref);
    let string_obj = string_obj.as_ref();
    let string_obj = string_obj.borrow();
    let string_obj = match string_obj.deref() {
        Ref::Obj(obj) => obj,
        _ => panic!("err")
    };

    let chars = string_obj.fields.iter()
        .find(|f| f.field.as_ref().name.eq("chars"))
        .unwrap();

    let chars = match &chars.value {
        Value::Ref(chars_ref) => chars_ref.clone(),
        _ => panic!("err")
    };

    let chars_arr = thread.object(chars);
    let chars_arr = chars_arr.as_ref();
    let chars_arr = chars_arr.borrow();
    let chars_arr = match chars_arr.deref() {
        Ref::Arr(arr) => arr,
        _ => panic!("err")
    };
    let chars_arr = match chars_arr {
        Array::Char(chars) => chars,
        _ => panic!("err")
    };

    let utf8_chars = String::from_utf16(chars_arr).unwrap();
    println!("{}", utf8_chars);

    None
}

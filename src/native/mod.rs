use crate::descriptor::MethodDescriptor;
use crate::heap::Value;
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

type NativeFunction = fn(thread: &mut Thread, args: Vec<Value>) -> Option<Value>;

fn robusta_println_string(thread: &mut Thread, args: Vec<Value>) -> Option<Value> {
    let runtime = thread.rt.borrow_mut();

    let string_ref = args[0].reference();
    let string_obj = runtime.load_object(string_ref);
    let string_obj = string_obj.as_ref().borrow();
    let string_obj = string_obj.obj();

    let chars = string_obj.fields.iter()
        .find(|f| f.field.as_ref().name.eq("chars"))
        .unwrap();
    let chars = chars.value.reference();

    let chars_arr = runtime.load_object(chars);
    let chars_arr = chars_arr.as_ref().borrow();
    let chars_array = chars_arr.arr().char();

    let utf8_chars = String::from_utf16(chars_array).unwrap();
    println!("{}", utf8_chars);

    None
}

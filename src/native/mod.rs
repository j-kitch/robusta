use std::str::FromStr;
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
                        },
                        NativeMethod {
                            name: String::from("parseBoolean"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/String;)Z"),
                            function: robusta_parse_boolean,
                        },
                        NativeMethod {
                            name: String::from("parseByte"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/String;)B"),
                            function: robusta_parse_byte,
                        },
                        NativeMethod {
                            name: String::from("parseChar"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/String;)C"),
                            function: robusta_parse_char,
                        },
                        NativeMethod {
                            name: String::from("parseShort"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/String;)S"),
                            function: robusta_parse_short,
                        },
                        NativeMethod {
                            name: String::from("parseInt"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/String;)I"),
                            function: robusta_parse_int,
                        },
                        NativeMethod {
                            name: String::from("parseLong"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/String;)L"),
                            function: robusta_parse_long,
                        },
                        NativeMethod {
                            name: String::from("parseFloat"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/String;)F"),
                            function: robusta_parse_float,
                        },
                        NativeMethod {
                            name: String::from("parseDouble"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/String;)D"),
                            function: robusta_parse_double,
                        },
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
    let string_ref = args[0].reference();
    let string = to_utf8_string(thread, string_ref);

    println!("{}", string);

    None
}

fn robusta_parse_boolean(thread: &mut Thread, args: Vec<Value>) -> Option<Value> {
    let string_ref = args[0].reference();
    let str_bool = to_utf8_string(thread, string_ref);
    let bool = bool::from_str(&str_bool).unwrap();

    let i32_bool: i32 = if bool { 1 } else { 0 };

    Some(Value::Int(i32_bool))
}

fn robusta_parse_byte(thread: &mut Thread, args: Vec<Value>) -> Option<Value> {
    let string_ref = args[0].reference();
    let str_byte = to_utf8_string(thread, string_ref);
    let byte = i8::from_str(&str_byte).unwrap();

    Some(Value::Int(byte as i32))
}

fn robusta_parse_char(thread: &mut Thread, args: Vec<Value>) -> Option<Value> {
    let string_ref = args[0].reference();
    let str_char = to_utf8_string(thread, string_ref);
    let char = u16::from_str(&str_char).unwrap();

    Some(Value::Int(char as i32))
}

fn robusta_parse_short(thread: &mut Thread, args: Vec<Value>) -> Option<Value> {
    let string_ref = args[0].reference();
    let str_short = to_utf8_string(thread, string_ref);
    let short = i16::from_str(&str_short).unwrap();

    Some(Value::Int(short as i32))
}

fn robusta_parse_int(thread: &mut Thread, args: Vec<Value>) -> Option<Value> {
    let string_ref = args[0].reference();
    let str_int = to_utf8_string(thread, string_ref);
    let int = i32::from_str(&str_int).unwrap();

    Some(Value::Int(int))
}

fn robusta_parse_long(thread: &mut Thread, args: Vec<Value>) -> Option<Value> {
    let string_ref = args[0].reference();
    let str_long = to_utf8_string(thread, string_ref);
    let long = i64::from_str(&str_long).unwrap();

    Some(Value::Long(long))
}

fn robusta_parse_float(thread: &mut Thread, args: Vec<Value>) -> Option<Value> {
    let string_ref = args[0].reference();
    let str_float = to_utf8_string(thread, string_ref);
    let float = f32::from_str(&str_float).unwrap();

    Some(Value::Float(float))
}

fn robusta_parse_double(thread: &mut Thread, args: Vec<Value>) -> Option<Value> {
    let string_ref = args[0].reference();
    let str_double = to_utf8_string(thread, string_ref);
    let double = f64::from_str(&str_double).unwrap();

    Some(Value::Double(double))
}

fn to_utf8_string(thread: &Thread, string_ref: u32) -> String {
    let runtime = thread.rt.borrow();

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

    String::from_utf16(chars_array).unwrap()
}

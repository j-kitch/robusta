use std::str::FromStr;

use crate::descriptor::MethodDescriptor;
use crate::heap::Value;
use crate::runtime::Runtime;

pub struct NativeMethods {
    classes: Vec<NativeClass>,
}

impl NativeMethods {
    pub fn load() -> Self {
        NativeMethods {
            classes: vec![
                NativeClass {
                    name: String::from("java/io/PrintStream"),
                    methods: vec![
                        NativeMethod {
                            name: String::from("println"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/String;)V"),
                            function: robusta_println_string,
                        },
                        NativeMethod {
                            name: String::from("println"),
                            descriptor: MethodDescriptor::parse("(Z)V"),
                            function: robusta_println_boolean,
                        },
                        NativeMethod {
                            name: String::from("println"),
                            descriptor: MethodDescriptor::parse("(I)V"),
                            function: robusta_println_int,
                        },
                        NativeMethod {
                            name: String::from("println"),
                            descriptor: MethodDescriptor::parse("(J)V"),
                            function: robusta_println_long,
                        },
                        NativeMethod {
                            name: String::from("println"),
                            descriptor: MethodDescriptor::parse("(F)V"),
                            function: robusta_println_float,
                        },
                        NativeMethod {
                            name: String::from("println"),
                            descriptor: MethodDescriptor::parse("(D)V"),
                            function: robusta_println_double,
                        },
                    ],
                },
                NativeClass {
                    name: String::from("java/lang/Integer"),
                    methods: vec![
                        NativeMethod {
                            name: String::from("parseInt"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/String;)I"),
                            function: robusta_parse_int,
                        },
                    ],
                },
                NativeClass {
                    name: String::from("java/lang/Long"),
                    methods: vec![
                        NativeMethod {
                            name: String::from("parseLong"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/String;)J"),
                            function: robusta_parse_long,
                        },
                    ],
                },
                NativeClass {
                    name: String::from("java/lang/Float"),
                    methods: vec![
                        NativeMethod {
                            name: String::from("parseFloat"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/String;)F"),
                            function: robusta_parse_float,
                        },
                    ],
                },
                NativeClass {
                    name: String::from("java/lang/Double"),
                    methods: vec![
                        NativeMethod {
                            name: String::from("parseDouble"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/String;)D"),
                            function: robusta_parse_double,
                        },
                    ],
                },
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

type NativeFunction = fn(runtime: &mut Runtime, args: Vec<Value>) -> Option<Value>;

fn robusta_println_string(runtime: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let string_ref = args[0].reference();
    let string = to_utf8_string(runtime, string_ref);

    println!("{}", string);

    None
}

fn robusta_parse_int(runtime: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let string_ref = args[0].reference();
    let str_int = to_utf8_string(runtime, string_ref);
    let int = i32::from_str(&str_int).unwrap();

    Some(Value::Int(int))
}

fn robusta_parse_long(runtime: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let string_ref = args[0].reference();
    let str_long = to_utf8_string(runtime, string_ref);
    let long = i64::from_str(&str_long).unwrap();

    Some(Value::Long(long))
}

fn robusta_parse_float(runtime: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let string_ref = args[0].reference();
    let str_float = to_utf8_string(runtime, string_ref);
    let float = f32::from_str(&str_float).unwrap();

    Some(Value::Float(float))
}

fn robusta_parse_double(runtime: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let string_ref = args[0].reference();
    let str_double = to_utf8_string(runtime, string_ref);
    let double = f64::from_str(&str_double).unwrap();

    Some(Value::Double(double))
}

fn robusta_println_boolean(_: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let bool = args[0].int() != 0;
    println!("{}", bool);

    None
}

fn robusta_println_int(_: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let int = args[0].int();
    println!("{}", int);

    None
}

fn robusta_println_long(_: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let long = args[0].long();
    println!("{}", long);

    None
}

fn robusta_println_float(_: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let float = args[0].float();
    println!("{}", float);

    None
}

fn robusta_println_double(_: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let double = args[0].double();
    println!("{}", double);

    None
}

fn to_utf8_string(runtime: &Runtime, string_ref: u32) -> String {
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

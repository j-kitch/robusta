use std::str::FromStr;

use crate::descriptor::MethodDescriptor;
use crate::heap::{Array, Value};
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
                            function: print_stream_println_string,
                        },
                        NativeMethod {
                            name: String::from("println"),
                            descriptor: MethodDescriptor::parse("(Z)V"),
                            function: print_stream_println_boolean,
                        },
                        NativeMethod {
                            name: String::from("println"),
                            descriptor: MethodDescriptor::parse("(B)V"),
                            function: print_stream_println_byte,
                        },
                        NativeMethod {
                            name: String::from("println"),
                            descriptor: MethodDescriptor::parse("(C)V"),
                            function: print_stream_println_char,
                        },
                        NativeMethod {
                            name: String::from("println"),
                            descriptor: MethodDescriptor::parse("(I)V"),
                            function: print_stream_println_int,
                        },
                        NativeMethod {
                            name: String::from("println"),
                            descriptor: MethodDescriptor::parse("(J)V"),
                            function: print_stream_println_long,
                        },
                        NativeMethod {
                            name: String::from("println"),
                            descriptor: MethodDescriptor::parse("(F)V"),
                            function: print_stream_println_float,
                        },
                        NativeMethod {
                            name: String::from("println"),
                            descriptor: MethodDescriptor::parse("(D)V"),
                            function: print_stream_println_double,
                        },
                    ],
                },
                NativeClass {
                    name: String::from("java/lang/Integer"),
                    methods: vec![
                        NativeMethod {
                            name: String::from("parseInt"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/String;)I"),
                            function: integer_parse_int,
                        },
                    ],
                },
                NativeClass {
                    name: String::from("java/lang/Long"),
                    methods: vec![
                        NativeMethod {
                            name: String::from("parseLong"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/String;)J"),
                            function: long_parse_long,
                        },
                    ],
                },
                NativeClass {
                    name: String::from("java/lang/Float"),
                    methods: vec![
                        NativeMethod {
                            name: String::from("parseFloat"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/String;)F"),
                            function: float_parse_float,
                        },
                    ],
                },
                NativeClass {
                    name: String::from("java/lang/Double"),
                    methods: vec![
                        NativeMethod {
                            name: String::from("parseDouble"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/String;)D"),
                            function: double_parse_double,
                        },
                    ],
                },
                NativeClass {
                    name: String::from("java/lang/System"),
                    methods: vec![
                        NativeMethod {
                            name: String::from("arraycopy"),
                            descriptor: MethodDescriptor::parse("(Ljava/lang/Object;ILjava/lang/Object;II)V"),
                            function: arraycopy,
                        }
                    ],
                }
            ]
        }
    }

    pub fn find_method(&self, class: &str, name: &str, descriptor: &MethodDescriptor) -> NativeFunction {
        self.classes.iter()
            .find(|c| c.name.eq(class))
            .expect(format!("Could not find class {}", class).as_str())
            .methods.iter()
            .find(|m| m.name.eq(name) && m.descriptor.eq(descriptor))
            .expect(format!("Could not find method {}.{}{}", class, name, descriptor.descriptor()).as_str())
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

fn print_stream_println_string(runtime: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let string_ref = args[1].reference();
    let string = to_utf8_string(runtime, string_ref);

    println!("{}", string);

    None
}

fn integer_parse_int(runtime: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let string_ref = args[0].reference();
    let str_int = to_utf8_string(runtime, string_ref);
    let int = i32::from_str(&str_int).unwrap();

    Some(Value::Int(int))
}

fn long_parse_long(runtime: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let string_ref = args[0].reference();
    let str_long = to_utf8_string(runtime, string_ref);
    let long = i64::from_str(&str_long).unwrap();

    Some(Value::Long(long))
}

fn float_parse_float(runtime: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let string_ref = args[0].reference();
    let str_float = to_utf8_string(runtime, string_ref);
    let float = f32::from_str(&str_float).unwrap();

    Some(Value::Float(float))
}

fn double_parse_double(runtime: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let string_ref = args[0].reference();
    let str_double = to_utf8_string(runtime, string_ref);
    let double = f64::from_str(&str_double).unwrap();

    Some(Value::Double(double))
}

fn print_stream_println_boolean(_: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let bool = args[1].int() != 0;
    println!("{}", bool);

    None
}

fn print_stream_println_byte(_: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let int = args[1].int();
    let bytes = int.to_be_bytes();
    let byte = i8::from_be_bytes([bytes[3]]);
    println!("{}", byte);

    None
}

fn print_stream_println_char(_: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let int = args[1].int();
    let bytes = int.to_be_bytes();
    let char = u16::from_be_bytes([bytes[2], bytes[3]]);
    let chars = vec![char];
    let string = String::from_utf16(&chars).unwrap();

    println!("{}", string);

    None
}

fn print_stream_println_int(_: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let int = args[1].int();
    println!("{}", int);

    None
}

fn print_stream_println_long(_: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let long = args[1].long();
    println!("{}", long);

    None
}

fn print_stream_println_float(_: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let double = args[1].float();

    if double.fract() == 0.0 {
        println!("{}.0", double);
    } else {
        println!("{}", double);
    }

    None
}

fn print_stream_println_double(_: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let double = args[1].double();

    if double.fract() == 0.0 {
        println!("{}.0", double);
    } else {
        println!("{}", double);
    }

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

fn arraycopy(runtime: &mut Runtime, args: Vec<Value>) -> Option<Value> {
    let src_ref = args[0].reference();
    let src_pos = args[1].int();
    let dest_ref = args[2].reference();
    let dest_pos = args[3].int();
    let length = args[4].int();

    let src = runtime.heap.get(src_ref);
    let src = src.as_ref().borrow();
    let src = src.arr();

    let dest = runtime.heap.get(dest_ref);
    let mut dest = dest.as_ref().borrow_mut();
    let dest = dest.arr_mut();

    match src {
        Array::Char(src) => {
            let dest = dest.char_mut();
            let src_i = src_pos as usize;
            let dest_i = dest_pos as usize;
            for i in 0..length as usize {
                dest[dest_i + i] = src[src_i + i];
            }
        },
        _ => panic!("arraycopy not supporting this type")
    }

    None
}

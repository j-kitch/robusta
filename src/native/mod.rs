use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use crate::descriptor::MethodDescriptor;
use crate::heap::{Array, Value};
use crate::native::class::ClassPlugin;
use crate::native::hash_code::HashCodePlugin;
use crate::native::static_plugin::Static;
use crate::runtime::Runtime;

mod hash_code;
mod class;
mod static_plugin;

pub trait NativePlugin {
    fn supports(&self, class: &str, name: &str, desc: &MethodDescriptor) -> bool;
    fn invoke(&mut self, runtime: &mut Runtime, args: Vec<Value>) -> Option<Value>;
}

pub struct NativeMethods {
    plugins: Vec<Rc<RefCell<dyn NativePlugin>>>,
}

impl NativeMethods {
    pub fn load() -> Self {
        NativeMethods {
            plugins: vec![
                Rc::new(RefCell::new(HashCodePlugin::new())),
                Rc::new(RefCell::new(ClassPlugin::new())),
                Rc::new(RefCell::new(Static::new("java/io/PrintStream", "println", MethodDescriptor::parse("(Ljava/lang/String;)V"), print_stream_println_string))),
                Rc::new(RefCell::new(Static::new("java/io/PrintStream", "println", MethodDescriptor::parse("(Z)V"), print_stream_println_boolean))),
                Rc::new(RefCell::new(Static::new("java/io/PrintStream", "println", MethodDescriptor::parse("(B)V"), print_stream_println_byte))),
                Rc::new(RefCell::new(Static::new("java/io/PrintStream", "println", MethodDescriptor::parse("(C)V"), print_stream_println_char))),
                Rc::new(RefCell::new(Static::new("java/io/PrintStream", "println", MethodDescriptor::parse("(I)V"), print_stream_println_int))),
                Rc::new(RefCell::new(Static::new("java/io/PrintStream", "println", MethodDescriptor::parse("(J)V"), print_stream_println_long))),
                Rc::new(RefCell::new(Static::new("java/io/PrintStream", "println", MethodDescriptor::parse("(F)V"), print_stream_println_float))),
                Rc::new(RefCell::new(Static::new("java/io/PrintStream", "println", MethodDescriptor::parse("(D)V"), print_stream_println_double))),
                Rc::new(RefCell::new(Static::new("java/lang/Integer", "parseInt", MethodDescriptor::parse("(Ljava/lang/String;)I"), integer_parse_int))),
                Rc::new(RefCell::new(Static::new("java/lang/Long", "parseLong", MethodDescriptor::parse("(Ljava/lang/String;)J"), long_parse_long))),
                Rc::new(RefCell::new(Static::new("java/lang/Float", "parseFloat", MethodDescriptor::parse("(Ljava/lang/String;)F"), float_parse_float))),
                Rc::new(RefCell::new(Static::new("java/lang/Double", "parseDouble", MethodDescriptor::parse("(Ljava/lang/String;)D"), double_parse_double))),
                Rc::new(RefCell::new(Static::new("java/lang/System", "arraycopy", MethodDescriptor::parse("(Ljava/lang/Object;ILjava/lang/Object;II)V"), arraycopy))),
            ],
        }
    }

    pub fn find_method(&self, class: &str, name: &str, descriptor: &MethodDescriptor) -> Rc<dyn Fn(&mut Runtime, Vec<Value>) -> Option<Value>> {
        let plugin = self.plugins.iter()
            .find(|p| {
                let p = p.as_ref().borrow();
                p.supports(class, name, descriptor)
            });

        let plugin = plugin.expect(format!("Could not find native method {}.{}{}", class, name, descriptor.descriptor()).as_str()).clone();


        Rc::new(move |rt, args| {
            let plugin = plugin.clone();
            let mut plugin = plugin.as_ref().borrow_mut();
            return plugin.invoke(rt, args);
        })
    }
}

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
        .find(|f| f.field.as_ref().name.eq("value"))
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
        }
        _ => panic!("arraycopy not supporting this type")
    }

    None
}

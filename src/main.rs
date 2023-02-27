use robusta::java;
use robusta::java::{FieldType, Value};

fn main() {
    let method_type = java::MethodType::from_descriptor("()Ljava/lang/String;").unwrap();
    let return_type: FieldType = method_type.returns.unwrap();
    let zero_value = return_type.zero_value();

    if let Value::Reference(reference) = zero_value {
        println!("Zero Value is {}", reference.0);
    }
}
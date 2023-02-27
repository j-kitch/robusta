use robusta::java;

fn main() {
    let field_type = java::FieldType::from_descriptor("Z").ok();
    println!("Got {:?}", field_type);
}
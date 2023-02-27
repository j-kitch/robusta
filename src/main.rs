use robusta::FieldType;

fn main() {
    let field_type = FieldType::from_descriptor("Z").ok();
    println!("Got {:?}", field_type);
}
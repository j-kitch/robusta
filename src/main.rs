use std::env::args;
use std::path::Path;
use robusta::{class_file};
use robusta::class_file::Const;
use robusta::java::{MethodType};

fn main() {
    let main_class = args().skip(1).next().unwrap();
    let main_class_location = Path::new("./classes").join(main_class).with_extension("class");

    let mut loader = class_file::loader::Loader::new(&main_class_location).unwrap();

    let class_file = loader.read_class_file().unwrap();

    let method_descriptor_idx = class_file.methods.get(0).unwrap().descriptor;
    let method_descriptor = class_file.const_pool.get(&method_descriptor_idx).unwrap();

    if let Const::Utf8 { bytes } = method_descriptor {
        let descriptor = String::from_utf8(bytes.clone()).unwrap();
        let descriptor = MethodType::from_descriptor(&descriptor).unwrap();
        println!("The descriptor of the first method is {:?}", descriptor);
    }
}
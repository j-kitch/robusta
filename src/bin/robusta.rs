use std::{env, format};
use std::fs::File;
use std::io::Read;
use robusta::class::Class;
use robusta::class_file::Reader;

fn main() {
    let main_class_name = env::args().nth(1).unwrap();
    let main_class = format!("{}.class", &main_class_name);
    let mut main_class_file = match File::open(&main_class) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Error: Could not find or load main class {}", &main_class_name);
            std::process::exit(1);
        }
    };

    let mut reader = Reader::new(&main_class_file);
    let class_file = reader.read_class_file();
    let class = Class::from(&class_file);

    println!("{:?}", class);
}

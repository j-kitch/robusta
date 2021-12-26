use std::{env, format};
use std::fs::File;

fn main() {
    let main_class_name = env::args().nth(1).unwrap();
    let main_class = format!("{}.class", &main_class_name);

    let main_class_file = File::open(&main_class);
    let _ = match main_class_file {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Error: Could not find or load main class {}", &main_class_name);
            std::process::exit(1);
        },
    };
}

use std::{env, format};
use std::fs::File;
use std::io::Read;

use robusta::class::Class;
use robusta::class_file::Reader;
use robusta::class_loader::ClassLoader;
use robusta::thread::{Frame, Thread};

fn main() {
    let main_class_name = env::args().nth(1).unwrap()
        .replace(".", "/");
    let mut loader = ClassLoader::new();

    loader.load("java/lang/String");

    let mut class = loader.load(&main_class_name);
    if class.is_none() {
        eprintln!("Error: Could not find or load main class {}", &main_class_name);
        std::process::exit(1);
    }
    let class = class.unwrap();
    let main = class.as_ref()
        .find_method("main", "([Ljava/lang/String;)V")
        .unwrap();
    let mut thread = Thread {
        frames: vec![
            Frame {
                pc: 0,
                class: class.clone(),
                method: main,
            }
        ]
    };

    thread.run();
}

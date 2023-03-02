use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use crate::class_file::ClassFile;
use crate::loader::parser::parse;

use crate::loader::source::{new_source, Source};

mod source;
mod parser;

pub struct Loader {
    source: Mutex<Box<dyn Source + Sync + Send>>,
}

impl Loader {
    pub fn new(class_path: Vec<PathBuf>) -> Arc<Self> {
        Arc::new(Loader { source: Mutex::new(new_source(class_path)) })
    }

    pub fn load(self: &Arc<Self>, class_name: &str) -> ClassFile {
        let mut source = self.source.lock().unwrap();
        let reader = source.open(class_name).unwrap();
        parse(reader)
    }
}
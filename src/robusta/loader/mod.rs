use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use crate::class_file::ClassFile;
use crate::loader::parser::parse;

use crate::loader::source::{Source, Sources};

mod source;
mod parser;

pub struct Loader {
    sources: Sources
}

impl Loader {
    pub fn new(class_path: Vec<PathBuf>) -> Arc<Self> {
        Arc::new(Loader { sources: Sources::new(class_path) })
    }

    pub fn load(self: &Arc<Self>, class_name: &str) -> ClassFile {
        self.sources.find(class_name).unwrap()
    }
}
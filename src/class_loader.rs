use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use crate::class::Class;
use crate::class_file::Reader;

// TODO: This is extremely brittle!
const CLASS_PATH: &str = "/Users/joshkitc/personal/robusta/java";

pub struct ClassLoader {
    loaded: HashMap<String, Class>
}

impl ClassLoader {
    pub fn new() -> Self {
        ClassLoader { loaded: HashMap::new() }
    }

    pub fn load(&mut self, class: &str) -> Option<&Class> {
        if !self.loaded.contains_key(class) {
            let file_name = Path::new(CLASS_PATH)
                .join(class)
                .with_extension("class");

            let file = File::open(file_name);
            if file.is_err() {
                return None;
            }
            let file = file.unwrap();
            let mut reader = Reader::new(&file);
            let class_file = reader.read_class_file();
            let class = Class::from(&class_file);
            self.loaded.insert(class.this_class.clone(), class);
        }
        self.loaded.get(class)
    }
}

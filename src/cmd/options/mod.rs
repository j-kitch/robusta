use std::collections::HashMap;
use crate::cmd::{Control, Robusta};

mod help;

pub struct Options {
    options: HashMap<String, OptionHandler>,
}

impl Options {
    pub fn new() -> Self {
        let mut options = Options {
            options: HashMap::new(),
        };

        options.options.insert("-cp".to_string(), non_std_set_class_path);
        options.options.insert("-classpath".to_string(), non_std_set_class_path);
        options.options.insert("-d32".to_string(), jvm_32_bit);
        options.options.insert("-d64".to_string(), non_op);
        options.options.insert("-?".to_string(), help::help);
        options.options.insert("-help".to_string(), help::help);

        options
    }

    pub fn find(&self, key: &str) -> Option<OptionHandler> {
        if key.starts_with('-') {
            return self.options.get(key).map(|f| *f)
        }

        None
    }
}

type OptionHandler = fn(&mut Robusta, &[String], usize) -> (Control, usize);

fn non_std_set_class_path(robusta: &mut Robusta, args: &[String], idx: usize) -> (Control, usize) {
    match args.get(idx + 1) {
        Some(value) => {
            robusta.configuration.class_path = value.to_string();
            (Control::Continue, idx + 2)
        }
        None => (Control::Error {
            error: format!("{} requires class path specification", args[0])
        }, idx + 1)
    }
}

fn non_op(_: &mut Robusta, _: &[String], idx: usize) -> (Control, usize) {
    (Control::Continue, idx + 1)
}

fn jvm_32_bit(_: &mut Robusta, _: &[String], idx: usize) -> (Control, usize) {
    (Control::Error {
        error: "Error: This Java instance does not support a 32-bit JVM.\nPlease install the desired version.".to_string(),
    }, idx + 1)
}

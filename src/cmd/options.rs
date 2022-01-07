use std::collections::HashMap;
use crate::cmd::{Control, Robusta};

pub struct Options {
    non_std: HashMap<String, OptionHandler>,
    std: HashMap<String, OptionHandler>,
}

impl Options {
    pub fn new() -> Self {
        let mut options = Options {
            non_std: HashMap::new(),
            std: HashMap::new(),
        };

        options.non_std.insert("-cp".to_string(), non_std_set_class_path);
        options.non_std.insert("-classpath".to_string(), non_std_set_class_path);

        options.std.insert("--class-path".to_string(), std_set_class_path);

        options
    }

    pub fn find(&self, key: &str) -> Option<OptionHandler> {
        if key.starts_with("--") {
            let mut key = key;
            if let Some(end) = key.find('=') {
                key = &key[..end];
            }
            return self.std.get(key).map(|f| *f)
        }

        if key.starts_with('-') {
            return self.non_std.get(key).map(|f| *f)
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

fn std_set_class_path(robusta: &mut Robusta, args: &[String], idx: usize) -> (Control, usize) {
    let first_arg = &args[idx];
    let eq_idx = first_arg.find('=');

    let val = eq_idx.map(|i| &first_arg[(i+1)..])
        .or(args.get(idx + 1).map(|s| s.as_str()));

    match val {
        Some(value) => {
            robusta.configuration.class_path = value.to_string();
            (Control::Continue, idx + 2)
        }
        None => (Control::Error {
            error: format!("{} requires class path specification", args[0])
        }, idx + 1)
    }
}

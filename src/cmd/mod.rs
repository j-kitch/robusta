use std::collections::HashMap;
use std::env::Args;

const CLASS_PATH_ENV_VAR: &str = "ROBUSTA_CLASSPATH";

pub struct Robusta {
    configuration: Configuration,
}

impl Robusta {
    pub fn new() -> Self {
        Robusta {
            configuration: Configuration {
                class_path: "".to_string(),
            }
        }
    }

    pub fn run(&mut self) {
        self.read_env();
        println!("{:?}", self.configuration)
    }

    fn read_env(&mut self) {
        let class_path = std::env::var(CLASS_PATH_ENV_VAR);
        if let Ok(class_path) = class_path {
            self.configuration.class_path = class_path;
        }
    }
}

#[derive(Debug)]
struct Configuration {
    class_path: String,
}


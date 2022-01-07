use std::env::args;
use crate::cmd::options::Options;

mod options;

const CLASS_PATH_ENV_VAR: &str = "ROBUSTA_CLASSPATH";

pub struct Robusta {
    configuration: Configuration,
    options: Options,
}

impl Robusta {
    pub fn new() -> Self {
        Robusta {
            configuration: Configuration {
                class_path: "".to_string(),
            },
            options: Options::new(),
        }
    }

    pub fn run(&mut self) -> Control {
        self.read_env();
        println!("{:?}", self.configuration);

        let args: Vec<String> = args().skip(1).collect();
        let mut i = 0;

        while i < args.len() {
            let first_arg = args.get(i).unwrap();
            let handler = match self.options.find(first_arg) {
                Some(handler) => handler,
                _ => return Control::Error {
                    error: format!("Unrecognized option: {}", first_arg),
                }
            };

            let (result, idx) = handler(self, &args, i);
            if let Control::Continue = result {
                i = idx;
            } else {
                return result;
            }
        }

        println!("{:?}", self.configuration);

        Control::Exit
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

pub enum Control {
    Continue,
    Exit,
    Error { error: String },
}

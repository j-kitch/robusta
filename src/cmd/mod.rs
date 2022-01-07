use std::env::args;
use crate::cmd::Control::Exit;
use crate::cmd::options::help::print_help;

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
                main_class: "".to_string(),
                main_args: Vec::new(),
            },
            options: Options::new(),
        }
    }

    pub fn run(&mut self) -> Control {
        self.read_env();

        let args: Vec<String> = args().skip(1).collect();
        let mut i = 0;

        while i < args.len() {
            let first_arg = args.get(i).unwrap();
            if !first_arg.starts_with("-") {
                break;
            }

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

        if i == args.len() {
            // No main class
            print_help();
            return Exit;
        }

        self.configuration.main_class = args.get(i).unwrap().clone();
        self.configuration.main_args = args.iter()
            .skip(i + 1)
            .map(Clone::clone)
            .collect();

        Exit
    }

    fn read_env(&mut self) {
        let class_path = std::env::var(CLASS_PATH_ENV_VAR);
        if let Ok(class_path) = class_path {
            self.configuration.class_path = class_path;
        }
    }

    fn run_main(&mut self) {

    }
}

#[derive(Debug)]
struct Configuration {
    class_path: String,
    main_class: String,
    main_args: Vec<String>,
}

pub enum Control {
    Continue,
    Exit,
    Error { error: String },
}

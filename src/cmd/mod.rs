use std::env::args;
use crate::cmd::Control::Exit;
use crate::cmd::options::help::print_help;

use crate::cmd::options::Options;
use crate::descriptor::MethodDescriptor;
use crate::heap::Value;
use crate::runtime::Runtime;
use crate::thread::Thread;

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

        self.run_main();

        Exit
    }

    fn read_env(&mut self) {
        let class_path = std::env::var(CLASS_PATH_ENV_VAR);
        if let Ok(class_path) = class_path {
            self.configuration.class_path = class_path;
        }
    }

    fn run_main(&mut self) {
        let mut runtime = Runtime::new(&self.configuration);

        let main_args_refs: Vec<u32> = self.configuration.main_args.iter()
            .map(|arg| runtime.insert_str_const(arg))
            .collect();
        let main_args_ref = runtime.insert_ref_array(main_args_refs);

        let main_class = runtime.load_class(&self.configuration.main_class);
        let main_method = main_class.find_method("main", &MethodDescriptor::parse("([Ljava/lang/String;)V")).unwrap();

        let mut main_thread = Thread::new(runtime);
        main_thread.create_frame(main_class, main_method, vec![Value::Ref(main_args_ref)]);

        main_thread.run()
    }
}

#[derive(Debug)]
pub struct Configuration {
    pub class_path: String,
    pub main_class: String,
    pub main_args: Vec<String>,
}

pub enum Control {
    Continue,
    Exit,
    Error { error: String },
}

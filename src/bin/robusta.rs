use std::process::exit;

use robusta::cmd::{Control, Robusta};

fn main() {
    let mut robusta = Robusta::new();
    let result = robusta.run();
    if let Control::Error { error } = result {
        eprintln!("{}", error);
        exit(1);
    }
}

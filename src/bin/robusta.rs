use std::env;
use robusta::cmd::Robusta;

use robusta::descriptor::MethodDescriptor;
use robusta::heap::Value;
use robusta::runtime::Runtime;
use robusta::thread::Thread;

fn main() {
    let mut robusta = Robusta::new();
    robusta.run()
}

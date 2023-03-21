use std::env::args;

use robusta::VirtualMachine;

fn main() {
    let mut jvm = VirtualMachine::new();

    jvm.start();
}
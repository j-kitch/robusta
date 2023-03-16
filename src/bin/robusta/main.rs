use std::env::args;

use robusta::VirtualMachine;

fn main() {
    let main_class = args().skip(1).next().unwrap();

    let mut jvm = VirtualMachine::new(&main_class);

    jvm.start();
}
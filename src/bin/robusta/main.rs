use std::env::args;
use tracing::info;

use robusta::VirtualMachine;

fn main() {
    info!("Starting JVM");

    let main_class = args().skip(1).next().unwrap();

    info!(class = main_class, "Found main class");

    let mut jvm = VirtualMachine::new(&main_class);

    jvm.start();
}
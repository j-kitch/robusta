use std::env::args;
use tracing::info;
use tracing::subscriber::set_global_default;

use robusta::VirtualMachine;

fn main() {
    let mut subscriber = tracing_subscriber::fmt()
        .with_thread_ids(true)
        .finish();
    // set_global_default(subscriber).unwrap();

    info!("Starting JVM");

    let main_class = args().skip(1).next().unwrap();

    info!(class = main_class, "Found main class");

    let mut jvm = VirtualMachine::new(&main_class);

    jvm.start();
}
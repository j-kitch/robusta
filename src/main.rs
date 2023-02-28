use std::env::args;

use robusta::java::MethodType;
use robusta::runtime::MethodArea;
use robusta::thread::Thread;

fn main() {
    let main_class = args().skip(1).next().unwrap();

    let method_area = MethodArea::new();

    let class = method_area.insert(&main_class);

    let mut main_thread = Thread::new(
        method_area.clone(),
        class.const_pool.clone(),
        class.methods.iter()
            .find(|m| m.name.eq("main") && m.descriptor.eq(&MethodType::from_descriptor("([Ljava/lang/String;)V").unwrap()))
            .unwrap()
            .clone());

    main_thread.run();
}
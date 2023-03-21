use std::path::PathBuf;

use criterion::{BenchmarkId, black_box, Criterion, criterion_group, criterion_main};
use robusta::java::MethodType;

use robusta::loader::{ClassFileLoader, Loader};
use robusta::method_area::const_pool::MethodKey;
use robusta::method_area::{Class, Method};
use robusta::runtime::Runtime;

pub fn load_benchmark(c: &mut Criterion) {
    let loader = ClassFileLoader::new(vec![
        PathBuf::from("./classes/rt.jar")
    ]);
    let loader = black_box(loader);

    let mut group = c.benchmark_group("Load class file from JAR");

    for name in ["java.lang.String", "java.lang.System", "java.util.concurrent.atomic.AtomicLong"] {
        group.bench_with_input(BenchmarkId::from_parameter(name), name, |b, name| {
            b.iter(|| loader.find(name))
        });
    }
}

pub fn load_class(c: &mut Criterion) {
    let mut group = c.benchmark_group("Load class object");
    let runtime = Runtime::new();

    for name in ["java.lang.String", "java.lang.Object", "java.util.concurrent.atomic.AtomicLong"] {
        group.bench_with_input(BenchmarkId::from_parameter(name), name, |b, name| {
            b.iter(|| {
                runtime.clear();
                runtime.method_area.load_outer_class(name)
            });
        });
    }
}

pub fn native_methods(c: &mut Criterion) {
    let mut group = c.benchmark_group("Find Native Method");
    let runtime = Runtime::new();

    let methods = [
        ("java.lang.Class", "registerNatives", "()V"),
        ("java.io.FileOutputStream", "writeBytes", "([BIIZ)V"),
        ("sun.misc.Unsafe", "arrayBaseOffset", "(Ljava/lang/Class;)I"),
    ];

    let classes: Vec<Class> = methods.iter().map(|(class, _, _)| {
        runtime.method_area.load_outer_class(class)
    }).collect();

    let methods: Vec<&Method> = methods.iter().enumerate().map(|(idx, (_, method, descriptor))| {
        let class = &classes[idx];
        class.find_method(&MethodKey {
            class: class.name().clone(),
            name: method.to_string(),
            descriptor: MethodType::from_descriptor(descriptor).unwrap(),
        }).unwrap()
    }).collect();

    for method in methods {
        group.bench_with_input(BenchmarkId::from_parameter(&method.name), method, |b, method| {
            b.iter(|| runtime.native.find(method));
        });
    }
}

criterion_group!(benches, load_benchmark);
criterion_group!(load_classes, load_class);
criterion_group!(natives, native_methods);
criterion_main!(benches, load_classes, natives);
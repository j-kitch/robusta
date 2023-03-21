use std::ops::Deref;
use std::path::PathBuf;

use criterion::{BenchmarkId, black_box, Criterion, criterion_group, criterion_main};

use robusta::java::{Int, MethodType};
use robusta::loader::{ClassFileLoader, Loader};
use robusta::method_area::{Class, Method};
use robusta::method_area::const_pool::MethodKey;
use robusta::runtime::Runtime;

pub fn load_benchmark(c: &mut Criterion) {
    let loader = ClassFileLoader::new(vec![
        PathBuf::from("./classes/rt.jar")
    ]);
    let loader = black_box(loader);

    let mut group = c.benchmark_group("Load class file from JAR");

    for name in ["java.lang.String", "java.lang.System", "java.util.concurrent.atomic.AtomicLong"] {
        group.bench_with_input(BenchmarkId::from_parameter(name), name, |b, name| {
            b.iter(|| black_box(loader.find(name)));
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
                black_box(runtime.method_area.load_outer_class(name));
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
            b.iter(|| black_box(runtime.native.find(method)));
        });
    }
}

fn allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Heap Allocation");
    let runtime = Runtime::new();

    for component in &[
        runtime.method_area.load_outer_class("byte"),
        runtime.method_area.load_outer_class("java.lang.String"),
        runtime.method_area.load_outer_class("long"),
        runtime.method_area.load_outer_class("int"),
    ] {
        for size in [10, 100, 1000, 10_000, 100_000] {
            let name = format!("Allocate array {} {}", &component.name(), size);
            let input: (&Class, i32) = (component, size);
            group.bench_with_input(BenchmarkId::from_parameter(name), &input, |b, input| {
                b.iter(|| {
                    runtime.heap.clear();
                    black_box(runtime.heap.new_array(input.0.clone(), Int(input.1)));
                });
            });
        }
    }

    for class in &[
        runtime.method_area.load_outer_class("java.lang.Object"),
        runtime.method_area.load_outer_class("java.lang.String"),
        runtime.method_area.load_outer_class("java.util.ArrayList"),
        runtime.method_area.load_outer_class("java.io.FileOutputStream"),
    ] {
        let name = format!("Allocate object {}", class.name());
        group.bench_with_input(BenchmarkId::from_parameter(name), class, |b, class| {
            b.iter(|| {
                runtime.heap.clear();
                black_box(runtime.heap.new_object(class.obj().deref()));
            })
        });
    }
}

criterion_group!(benches, load_benchmark);
criterion_group!(load_classes, load_class);
criterion_group!(natives, native_methods);
criterion_group!(allocate, allocation);
criterion_main!(benches, load_classes, natives, allocate);
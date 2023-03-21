use std::path::PathBuf;
use criterion::{BenchmarkId, black_box, Criterion, criterion_group, criterion_main};
use robusta::loader::{ClassFileLoader, Loader};

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

criterion_group!(benches, load_benchmark);
criterion_main!(benches);
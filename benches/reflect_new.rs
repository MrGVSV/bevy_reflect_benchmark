use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use bevy_reflect_benchmark::{old, new};

pub fn bench_large_struct(c: &mut Criterion) {
    let mut group = c.benchmark_group("Large Non-generic Struct");

    let test_struct = new::prepare(new::NonGenericBigBoi::default());

    group.bench_with_input(BenchmarkId::new("Field (First)", "New"), &test_struct, |b, test_struct| {
        b.iter(|| {
            new::field(test_struct.as_ref(), black_box("field_000"))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field (Middle)", "New"), &test_struct, |b, test_struct| {
        b.iter(|| {
            new::field(test_struct.as_ref(), black_box("field_050"))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field (Last)", "New"), &test_struct, |b, test_struct| {
        b.iter(|| {
            new::field(test_struct.as_ref(), black_box("field_100"))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field At (First)", "New"), &test_struct, |b, test_struct| {
        b.iter(|| {
            new::field_at(test_struct.as_ref(), black_box(0))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field At (Middle)", "New"), &test_struct, |b, test_struct| {
        b.iter(|| {
            new::field_at(test_struct.as_ref(), black_box(50))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field At (Last)", "New"), &test_struct, |b, test_struct| {
        b.iter(|| {
            new::field_at(test_struct.as_ref(), black_box(100))
        })
    });

    let test_struct = old::prepare(old::NonGenericBigBoi::default());

    group.bench_with_input(BenchmarkId::new("Field (First)", "Old"), &test_struct, |b, test_struct| {
        b.iter(|| {
            old::field(test_struct.as_ref(), black_box("field_000"))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field (Middle)", "Old"), &test_struct, |b, test_struct| {
        b.iter(|| {
            old::field(test_struct.as_ref(), black_box("field_050"))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field (Last)", "Old"), &test_struct, |b, test_struct| {
        b.iter(|| {
            old::field(test_struct.as_ref(), black_box("field_100"))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field At (First)", "Old"), &test_struct, |b, test_struct| {
        b.iter(|| {
            old::field_at(test_struct.as_ref(), black_box(0))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field At (Middle)", "Old"), &test_struct, |b, test_struct| {
        b.iter(|| {
            old::field_at(test_struct.as_ref(), black_box(50))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field At (Last)", "Old"), &test_struct, |b, test_struct| {
        b.iter(|| {
            old::field_at(test_struct.as_ref(), black_box(100))
        })
    });
    group.finish();

    let mut group = c.benchmark_group("Large Generic Struct");

    let test_struct = new::prepare(<new::GenericBigBoi<i32, usize, f32>>::default());

    group.bench_with_input(BenchmarkId::new("Field (First)", "New"), &test_struct, |b, test_struct| {
        b.iter(|| {
            new::field(test_struct.as_ref(), black_box("field_000"))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field (Middle)", "New"), &test_struct, |b, test_struct| {
        b.iter(|| {
            new::field(test_struct.as_ref(), black_box("field_050"))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field (Last)", "New"), &test_struct, |b, test_struct| {
        b.iter(|| {
            new::field(test_struct.as_ref(), black_box("field_100"))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field At (First)", "New"), &test_struct, |b, test_struct| {
        b.iter(|| {
            new::field_at(test_struct.as_ref(), black_box(0))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field At (Middle)", "New"), &test_struct, |b, test_struct| {
        b.iter(|| {
            new::field_at(test_struct.as_ref(), black_box(50))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field At (Last)", "New"), &test_struct, |b, test_struct| {
        b.iter(|| {
            new::field_at(test_struct.as_ref(), black_box(100))
        })
    });

    let test_struct = old::prepare(<old::GenericBigBoi<i32, usize, f32>>::default());

    group.bench_with_input(BenchmarkId::new("Field (First)", "Old"), &test_struct, |b, test_struct| {
        b.iter(|| {
            old::field(test_struct.as_ref(), black_box("field_000"))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field (Middle)", "Old"), &test_struct, |b, test_struct| {
        b.iter(|| {
            old::field(test_struct.as_ref(), black_box("field_050"))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field (Last)", "Old"), &test_struct, |b, test_struct| {
        b.iter(|| {
            old::field(test_struct.as_ref(), black_box("field_100"))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field At (First)", "Old"), &test_struct, |b, test_struct| {
        b.iter(|| {
            old::field_at(test_struct.as_ref(), black_box(0))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field At (Middle)", "Old"), &test_struct, |b, test_struct| {
        b.iter(|| {
            old::field_at(test_struct.as_ref(), black_box(50))
        })
    });
    group.bench_with_input(BenchmarkId::new("Field At (Last)", "Old"), &test_struct, |b, test_struct| {
        b.iter(|| {
            old::field_at(test_struct.as_ref(), black_box(100))
        })
    });
    group.finish();
}

criterion_group!(benches, bench_large_struct);
criterion_main!(benches);
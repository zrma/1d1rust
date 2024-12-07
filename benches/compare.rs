use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn sum_for(vec: &[u64]) -> u64 {
    let mut acc = 0;
    for &val in vec {
        acc += val;
    }
    acc
}

fn sum_map(vec: &[u64]) -> u64 {
    vec.iter().sum()
}

fn sum_fold(vec: &[u64]) -> u64 {
    vec.iter().fold(0, |acc, &x| acc + x)
}

fn compare_bench(c: &mut Criterion) {
    let n = 100_000_000;
    let vec: Vec<u64> = (0..n).map(|x| x as u64).collect();

    c.bench_function("sum_for", |b| {
        b.iter(|| {
            let result = sum_for(black_box(&vec));
            black_box(result);
        })
    });

    c.bench_function("sum_map", |b| {
        b.iter(|| {
            let result = sum_map(black_box(&vec));
            black_box(result);
        })
    });

    c.bench_function("sum_fold", |b| {
        b.iter(|| {
            let result = sum_fold(black_box(&vec));
            black_box(result);
        })
    });
}

criterion_group!(benches, compare_bench);
criterion_main!(benches);

use criterion::{Criterion, criterion_group, criterion_main};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use spaced_list_5::HollowSpacedList;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("push and insert");
    group.bench_function("push", |b| b.iter(|| {
        let mut list = HollowSpacedList::new();
        for _ in 0..50_000 {
            list.try_push(1).unwrap();
        }
    }));
    group.bench_function("insert", |b| b.iter(|| {
        let mut list = HollowSpacedList::new();
        for n in 0..50_000 {
            list.insert(n);
        }
    }));
    group.bench_function("insert random", |b| b.iter(|| {
        let mut list = HollowSpacedList::new();
        let mut rng = StdRng::seed_from_u64(0);
        for _ in 0..50_000 {
            list.insert(rng.gen_range(-1_000_000..1_000_000));
        }
    }));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
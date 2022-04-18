use criterion::{black_box, Criterion, criterion_group, criterion_main};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use spaced_list_5::HollowSpacedList;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("append and insert");
    group.bench_function("append 10k", |b| b.iter(|| {
        let mut list = HollowSpacedList::new();
        for _ in 0..10_000 {
            list.append_node(1);
        }
    }));
    group.bench_function("insert 10k", |b| b.iter(|| {
        let mut list = HollowSpacedList::new();
        for n in 0..10_000 {
            list.insert_node(n);
        }
    }));
    group.bench_function("insert random 10k", |b| b.iter(|| {
        let mut list = HollowSpacedList::new();
        let mut rng = StdRng::seed_from_u64(0);
        for _ in 0..10_000 {
            list.insert_node(rng.gen_range(-1_000_000..1_000_000));
        }
    }));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
use criterion::{Criterion, criterion_group, criterion_main};
use spaced_list_5::HollowSpacedList;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("append_node");
    group.bench_function("append_node", |b| b.iter(|| {
        let mut list = HollowSpacedList::new();
        for _ in 0..100_000 {
            list.append_node(1);
        }
    }));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
use std::iter::repeat_with;
use std::time::Duration;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use itertools::Itertools;
use spaced_list_5::HollowSpacedList;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("before");
    for e in (12..=32).step_by(8) {
        let n: u32 = (1 << e) + (1 << (e - 2));

        let copies = if e <= 20 { (1 << 16 << 4) / (1 << e) } else { 1 };
        let lists = repeat_with(|| {
            let mut list = HollowSpacedList::new();
            for _ in 0..n {
                list.push(1);
            }
            list
        }).take(copies as usize).collect_vec();

        let mut list_index = 0;
        let mut pos = 0u32;
        let step = (1 << 16) + 1;
        group.bench_with_input(BenchmarkId::from_parameter(e), &lists, |b, lists| b.iter(|| {
            let list = &lists[list_index];
            list_index += 3;
            list_index %= lists.len();
            list.before(pos % n);
            pos = pos.wrapping_add(step);
        }));
    }
    group.finish();
}

// criterion_group!(benches, criterion_benchmark);
criterion_group!{
    name = benches;
    config = Criterion::default().warm_up_time(Duration::from_secs(3)).measurement_time(Duration::from_secs(5)).noise_threshold(0.04);
    targets = criterion_benchmark
}
criterion_main!(benches);
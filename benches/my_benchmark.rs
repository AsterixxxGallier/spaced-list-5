use criterion::{black_box, Criterion, criterion_group, criterion_main};

fn old_(n: u32) -> u32 {
    let mut link_index = n;
    for degree in 0..16 {
        let bit = 1 << degree;
        if link_index & bit == 0 {
            link_index += bit;
        }
    }
    link_index
}

fn old(n: u32) -> u32 {
    let mut link_index = n;
    let mut degree = 0;
    while degree < 16 {
        let bit = 1 << degree;
        if link_index & bit == 0 {
            link_index += bit;
        }
        degree += 1;
    }
    link_index
}

fn new(n: u32) -> u32 {
    let mut link_index = n;
    let mut degree = 0;
    while degree < 16 {
        degree += (link_index >> degree).trailing_ones();
        link_index += 1 << degree;
        degree += 1;
    }
    link_index
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut counter: u16 = 0;
    c.bench_function("old", |b| b.iter(|| {
        old(black_box(counter as u32));
        counter += 1;
    }));
    c.bench_function("new", |b| b.iter(|| {
        new(black_box(counter as u32));
        counter += 1;
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

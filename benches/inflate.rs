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
    let mut counter = 0;
    let mut link_index = n;
    let mut degree = 0;
    while degree < 16 {
        let bit = 1 << degree;
        if n & bit == 0 {
            counter += link_index;
            link_index += bit;
        }
        degree += 1;
    }
    counter
}

fn trailing_ones(n: u32) -> u32 {
    let mut counter = 0;
    let mut link_index = n;
    let mut degree = 0;
    while degree < 16 {
        degree += (link_index >> degree).trailing_ones();
        counter += link_index;
        link_index += 1 << degree;
        degree += 1;
    }
    counter
}

#[inline(always)]
const fn link_index(node_index: u32, degree: u32) -> u32 {
    node_index + (1 << degree) - 1
}

fn node_index(n: u32) -> u32 {
    let mut counter = 0;
    let mut node_index = n;
    for degree in 0..16 {
        if node_index & 1 == 0 {
            counter += link_index(node_index << degree, degree);
        }
        node_index >>= 1;
    }
    counter
}

fn node_index_opt(n: u32) -> u32 {
    let mut counter = 0;
    for degree in 0..16 {
        if n >> degree & 1 == 0 {
            counter += link_index(n >> degree << degree, degree);
        }
    }
    counter
}

#[inline(always)]
const fn link_index_2(node_index: u32, degree: u32) -> u32 {
    node_index | ((1 << degree) - 1)
}

fn node_index_opt_2(n: u32) -> u32 {
    let mut counter = 0;
    for degree in 0..16 {
        if n >> degree & 1 == 0 {
            counter += link_index_2(n, degree);
        }
    }
    counter
}

fn node_index_trailing_ones(n: u32) -> u32 {
    let mut counter = 0;
    let mut node_index = n;
    let mut degree = 0;
    while degree < 16 {
        degree += node_index.trailing_ones();
        counter += link_index(node_index << degree, degree);
        node_index >>= 1;
        degree += 1;
    }
    counter
}

fn criterion_benchmark(c: &mut Criterion) {
    println!("{}", old(23));
    println!("{}", trailing_ones(23));
    println!("{}", trailing_ones(23));
    // let mut counter: u16 = 0;
    // let mut group = c.benchmark_group("links_above");
    // group.bench_function("old", |b| b.iter(|| {
    //     black_box(old(black_box(counter as u32)));
    //     counter += 1;
    // }));
    // group.bench_function("trailing_ones", |b| b.iter(|| {
    //     black_box(trailing_ones(black_box(counter as u32)));
    //     counter += 1;
    // }));
    // group.bench_function("node_index", |b| b.iter(|| {
    //     black_box(node_index(black_box(counter as u32)));
    //     counter += 1;
    // }));
    // group.bench_function("node_index_opt", |b| b.iter(|| {
    //     black_box(node_index_opt(black_box(counter as u32)));
    //     counter += 1;
    // }));
    // group.bench_function("node_index_opt_2", |b| b.iter(|| {
    //     black_box(node_index_opt_2(black_box(counter as u32)));
    //     counter += 1;
    // }));
    // group.bench_function("node_index_trailing_ones", |b| b.iter(|| {
    //     black_box(node_index_trailing_ones(black_box(counter as u32)));
    //     counter += 1;
    // }));
    // group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

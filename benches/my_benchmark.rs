use criterion::{criterion_group, criterion_main, Criterion};
use complewise_iter::{IntoComplewiseIterator, LendingIterator};

fn test_complewise_iter() {
    let mut items = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20];
    let mut iter = items.complewise();
    while let Some((item, others)) = iter.next() {
        *item += others.into_iter().sum::<i32>();
    }
}


fn test_c_style() {
    let mut items = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20];
    let ptr = items.as_mut_ptr();
    for i in 0..items.len() {
        unsafe {
            let item: &mut i32 = ptr.add(i).as_mut().unwrap();
            for j in 0..items.len() {
                let other: &i32 = ptr.add(j).as_ref().unwrap();
                *item += *other;
            }
        }
    }
}

/// Benchmark the complewise_iter method vs a c-style method.
/// Both functions add the sum of the other elements to each element in the list, one at a time,
/// where the effect of the previous sum will effect the next sum.
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test_complewise_iter", |b| b.iter(|| test_complewise_iter()));
    c.bench_function("test_c_style", |b| b.iter(|| test_c_style()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
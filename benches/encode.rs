use chunk_diff::{
    encoder::{Config, Encoder},
    hasher::XxHasher,
    util::rect::Rect,
};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::Rng;

fn criterion_benchmark(c: &mut Criterion) {
    let data = vec![0; 3840 * 2160];
    let rect = Rect::new(320, 180);
    let full = Rect::new(3840, 2160);
    let config = Config::new(rect, full);
    let mut rng = rand::thread_rng();
    let mut encoder = Encoder::<XxHasher>::new(config);
    let _ = encoder.encode(&data);

    c.bench_function("encode 4k", |b| {
        b.iter_batched(
            || {
                let mut data = vec![0; 3840 * 2160];
                (0..10).for_each(|_| {
                    let start = rng.gen_range(0..3840 * 2160 - 10);
                    data[start..start + 10]
                        .copy_from_slice(&(0..10).map(|_| rng.gen::<u8>()).collect::<Vec<_>>());
                });
                data
            },
            |data| {
                let _ = encoder.encode(&data);
            },
            BatchSize::LargeInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

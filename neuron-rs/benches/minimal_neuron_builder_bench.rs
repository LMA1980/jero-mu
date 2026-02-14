use criterion::{black_box, criterion_group, criterion_main, Criterion};
use neuron_rs::NeuronBuilder;

fn benchmark_builder(c: &mut Criterion) {
    c.bench_function("build_neuron_minimal", |b| {
        b.iter(|| {
            // We use black_box to prevent the compiler from 
            // optimizing away our code since the result isn't "used"
            let _ = black_box(NeuronBuilder::new())
                .with_id(black_box("neuron-01"))
                .build();
        })
    });
}

criterion_group!(benches, benchmark_builder);
criterion_main!(benches);

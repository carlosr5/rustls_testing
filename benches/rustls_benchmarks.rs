use criterion::BenchmarkId;
use criterion::{criterion_group, criterion_main, Criterion};

use rustls_benchmarking::{start_client, start_platform_verifier_client};


fn bench_client_rtts(c: &mut Criterion) {
    // Here is where you can change the number of samples that you'd like to run!
    let num_samples = 1000;

    let mut group = c.benchmark_group("Client RTT");
    group.sample_size(num_samples);
    group.bench_function(BenchmarkId::new("rustls client rtt", num_samples), |b| b.iter(|| start_client()));
    group.bench_function(BenchmarkId::new("platform verifier client rtt", num_samples), |b| b.iter(|| start_platform_verifier_client()));
    group.finish();
}

criterion_group!(benches, bench_client_rtts);
criterion_main!(benches);

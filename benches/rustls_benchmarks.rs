use criterion::BenchmarkId;
use criterion::{criterion_group, criterion_main, Criterion};
use std::env;

use rustls_benchmarking::start_client;

/**
 * This function is used to run the benchmarks for the client round trip time (RTT) for webpki and rustls-platform-verifier certificate validation.
 * We measure the RTT by timing how long it takes for the client to start, connect to the server, and then disconnect.
 */
fn bench_client_rtts(c: &mut Criterion) {
    // Here is where you can change the number of samples that you'd like to run!
    let num_samples = env::var("NUM_SAMPLES")
        .unwrap_or("100".to_string())
        .parse::<usize>()
        .expect("NUM_SAMPLES must be a valid usize");

    // Creating a testing group for us to run our benchmarks in.
    let mut group = c.benchmark_group("Client RTT");
    group.sample_size(num_samples);

    // Now, we'll add each of the functions we want to compare against one another to the testing group.
    group.bench_function(BenchmarkId::new("rustls-platform-verifier client rtt", num_samples), |b| b.iter(|| start_client("rustls-platform-verifier".to_string())));
    group.bench_function(BenchmarkId::new("rustls webpki client rtt", num_samples), |b| b.iter(|| start_client("webpki".to_string())));
    group.bench_function(BenchmarkId::new("rust-native-tls client rtt", num_samples), |b| b.iter(|| start_client("rust-native-tls".to_string())));

    // Finally we'll finish the group and make Criterion generate the test reports.
    group.finish();
}

criterion_group!(benches, bench_client_rtts);
criterion_main!(benches);

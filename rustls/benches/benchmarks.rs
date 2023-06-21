#[path = "../tests/common/mod.rs"]
mod test_utils;
use criterion::BenchmarkId;
use test_utils::*;

use rustls::ServerConnection;
use rustls::start_client;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::io;
use std::sync::Arc;

fn bench_ewouldblock(c: &mut Criterion) {
    let server_config = make_server_config(KeyType::Rsa);
    let mut server = ServerConnection::new(Arc::new(server_config)).unwrap();
    let mut read_ewouldblock = FailsReads::new(io::ErrorKind::WouldBlock);
    c.bench_function("rustls tests", |b| b.iter(|| server.read_tls(&mut read_ewouldblock)));
}

fn bench_client_rtt(c: &mut Criterion) {
    // Here is where you can change the number of samples that you'd like to run!
    let num_samples = 100;

    let mut group = c.benchmark_group("client_rtt");
    group.sample_size(num_samples);
    group.bench_function(BenchmarkId::new("rustls client rtt", num_samples), |b| b.iter(|| start_client()));
    group.finish();
}

criterion_group!(benches, bench_client_rtt, bench_ewouldblock);
criterion_main!(benches);

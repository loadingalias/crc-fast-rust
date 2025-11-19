use crc_fast::{checksum_with_params, CrcAlgorithm, CrcKeysStorage, CrcParams};
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::hint::black_box;

fn benchmark_custom_crc(c: &mut Criterion) {
    let params = CrcParams {
        algorithm: CrcAlgorithm::Crc32Custom,
        name: "Custom",
        width: 32,
        poly: 0x04c11db7,
        init: 0xffffffff,
        refin: true,
        refout: true,
        xorout: 0xffffffff,
        check: 0,
        keys: CrcKeysStorage::KeysFold256([0; 23]),
    };

    let data = vec![0u8; 1024]; // 1KB

    let mut group = c.benchmark_group("software_fallback");
    group.throughput(Throughput::Bytes(data.len() as u64));

    group.bench_function("custom_crc32_1kb", |b| {
        b.iter(|| checksum_with_params(black_box(params), black_box(&data)))
    });

    group.finish();
}

criterion_group!(benches, benchmark_custom_crc);
criterion_main!(benches);

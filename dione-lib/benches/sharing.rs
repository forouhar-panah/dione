extern crate dione_lib;

use criterion::{Criterion, criterion_group, criterion_main, Throughput};

use dione_lib::cryptography::sharing::SharingAlgorithm;

fn shamir_bench(c: &mut Criterion) {
	const T: u8 = 5;
	const N: usize = 10;
	{
		const SIZE: usize = 1_000;

		let mut group = c.benchmark_group("sss-1000b");
		group.throughput(Throughput::Bytes(SIZE as u64));
		group.bench_function("sss-1000b", |bencher| {
			bencher.iter(|| {
				let algo = dione_lib::cryptography::sharing::shamir::SSS;
				let shares = algo.share(&[0; SIZE], N, T);
				algo.reconstruct(&shares, T).unwrap();
			});
		});
	}
	{
		const SIZE: usize = 1_000_000;

		let mut group = c.benchmark_group("sss-1MB");
		group.sample_size(10);
		group.throughput(Throughput::Bytes(SIZE as u64));
		group.bench_function("sss-1MB", |bencher| {
			bencher.iter(|| {
				let algo = dione_lib::cryptography::sharing::shamir::SSS;
				let shares = algo.share(&[0; SIZE], N, T);
				algo.reconstruct(&shares, T).unwrap();
			});
		});
	}
}

criterion_group!(benches, shamir_bench);
criterion_main!(benches);

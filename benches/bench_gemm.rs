use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_gemm(m: u64, n: u64, k: u64, a: &Vec<f32>, b: &Vec<f32>, c: &mut Vec<f32>) {
    hasty::blas::level3::gemm(
        hasty::blas::StorageOrder::RowMajor,
        hasty::blas::Transpose::NoTrans,
        hasty::blas::Transpose::NoTrans,
        m,
        n,
        k,
        1.0,
        a,
        k,
        b,
        n,
        0.0,
        c,
        n,
    );
}

fn criterion_benchmark(crit: &mut Criterion) {
    let mut mnk = crit.benchmark_group(format!("gemm ({})", hasty::blas::get_blas_library()));

    for size in [5, 10, 20, 50, 100, 200, 500, 1000, 1500, 2000].iter() {
        let m: u64 = *size;
        let n: u64 = *size;
        let k: u64 = *size;

        let mut a = vec![0.0; (m * k) as usize];
        let mut b = vec![0.0; (k * n) as usize];
        let mut c = vec![0.0; (m * n) as usize];

        for i in 0..(m * k) {
            a[i as usize] = (i + 1) as f32;
        }

        for i in 0..(k * n) {
            b[i as usize] = (i + 1) as f32;
        }

        mnk.bench_with_input(
            criterion::BenchmarkId::new("m=n=k", size),
            size,
            |bench, &size| {
                bench.iter(|| {
                    bench_gemm(
                        black_box(size),
                        black_box(size),
                        black_box(size),
                        &a,
                        &b,
                        &mut c,
                    )
                })
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

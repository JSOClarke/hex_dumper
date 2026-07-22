use criterion::{Criterion, black_box};
use hex_dumper::convert_chunk;
use hex_dumper::convert_chunk_v2;

fn bench_convert_chunk(c: &mut Criterion){
    //Sample input data for benchmarking
    let data: &[u8] = b"Hello, World! This is a sample input for benchmarking the convert_chunk function.";

    c.bench_function("convert_chunk", |b| {
        b.iter(|| {
            black_box(convert_chunk(black_box(&data)));
        });
    });
}

fn bench_convert_chunk_v2(c: &mut Criterion){
    //Sample input data for benchmarking
    let data: &[u8] = b"Hello, World! This is a sample input for benchmarking the convert_chunk function.";
    let mut hex_repr_buf = String::new();
    let mut asci_repr_buf = String::new();

    c.bench_function("convert_chunk_v2", |b| {
        b.iter(|| {
            black_box(convert_chunk_v2(black_box(&data), black_box(&mut hex_repr_buf), black_box(&mut asci_repr_buf)));
        });
    });
}



criterion::criterion_group!(benches, bench_convert_chunk, bench_convert_chunk_v2);
criterion::criterion_main!(benches);    
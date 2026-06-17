use divan::black_box;

fn main() {
    // Run the benchmarks
    divan::main();
}

// Benchmark allocating on the heap vs stack
#[divan::bench]
fn bench_heap() {
    let _a = black_box(vec![0; 1000]);
}

#[divan::bench]
fn bench_stack() {
    let _a = black_box([0; 1000]);
}
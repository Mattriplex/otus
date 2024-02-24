use criterion::{criterion_group, criterion_main, Criterion};
use otus::{board::Board, search::minimax::search_minimax};

pub fn minimax_benchmark(c: &mut Criterion) {
    let mut board = Board::default();
    c.bench_function("minimax", |b| b.iter(|| search_minimax(&mut board, 3)));
}
//baseline 194ms
//moving noise up to outer search function -> 169ms
//improve knight hop iter -> 154ms
//killing other processes -> 97ms
//eliminating heap allocations for pseudo move generation -> 5ms

pub fn minimax_benchmark_big(c: &mut Criterion) {
    let mut board = Board::default();
    c.bench_function("minimax_big", |b| b.iter(|| search_minimax(&mut board, 4)));
}
//baseline (after above optimizations) 207ms

criterion_group!(benches, minimax_benchmark, minimax_benchmark_big);
criterion_main!(benches);

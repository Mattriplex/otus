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

criterion_group!(benches, minimax_benchmark);
criterion_main!(benches);

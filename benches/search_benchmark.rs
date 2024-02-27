use criterion::{criterion_group, criterion_main, Criterion};
use otus::{
    board::Board,
    search::{eval::{get_material_eval, smart_eval}, minimax::{search_alpha_beta, search_minimax}},
};

pub fn minimax_benchmark_smart(c: &mut Criterion) {
    let mut board = Board::default();
    c.bench_function("minimax_smart", |b| b.iter(|| search_minimax(&mut board, 4, smart_eval)));
}

pub fn minimax_benchmark(c: &mut Criterion) {
    let mut board = Board::default();
    c.bench_function("minimax", |b| b.iter(|| search_minimax(&mut board, 3, get_material_eval)));
}
//baseline 194ms
//moving noise up to outer search function -> 169ms
//improve knight hop iter -> 154ms
//killing other processes -> 97ms
//eliminating heap allocations for pseudo move generation -> 5ms

pub fn minimax_benchmark_big(c: &mut Criterion) {
    let mut board = Board::default();
    c.bench_function("minimax_big", |b| b.iter(|| search_minimax(&mut board, 4, get_material_eval)));
}
//baseline (after above optimizations) 207ms
//remove duplicate legality check: 140ms
//other optimizations: 80ms

pub fn alpha_beta_benchmark(c: &mut Criterion) {
    let mut board = Board::default();
    c.bench_function("alpha_beta", |b| {
        b.iter(|| search_alpha_beta(&mut board, 4))
    });
}

criterion_group!(
    benches,
    minimax_benchmark,
    minimax_benchmark_big,
    minimax_benchmark_smart,
    alpha_beta_benchmark
);
criterion_main!(benches);

use criterion::{criterion_group, criterion_main, Criterion};
use otus::{
    board::Board,
    hashing::TranspTable,
    search::{
        eval::{get_material_eval, smart_eval},
        minimax::{search_alpha_beta, search_minimax, search_minimax_cached},
    },
};

pub fn minimax_cached(c: &mut Criterion) {
    let mut board = Board::default();

    c.bench_function("minimax_cached", |b| {
        b.iter(|| {
            let mut transp_table = TranspTable::new(1 << 20);
            search_minimax_cached(&mut board, 4, smart_eval, &mut transp_table)
        });
    });
}

pub fn minimax_benchmark(c: &mut Criterion) {
    let mut board = Board::default();
    c.bench_function("minimax", |b| {
        b.iter(|| search_minimax(&mut board, 3, smart_eval))
    });
}
//baseline 194ms
//moving noise up to outer search function -> 169ms
//improve knight hop iter -> 154ms
//killing other processes -> 97ms
//eliminating heap allocations for pseudo move generation -> 5ms

pub fn minimax_benchmark_big(c: &mut Criterion) {
    let mut board = Board::default();
    c.bench_function("minimax_big", |b| {
        b.iter(|| search_minimax(&mut board, 4, smart_eval))
    });
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
    minimax_cached,
    alpha_beta_benchmark
);
criterion_main!(benches);

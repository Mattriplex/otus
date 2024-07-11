use rand::Rng;

pub mod benchmark;
pub mod optimized;

fn get_noise() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(-0.1..0.1)
}

struct SearchResult {
    eval: f32,
    nodes_searched: u64,
}

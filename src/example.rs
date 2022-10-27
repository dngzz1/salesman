use super::block::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Creates a vector of random points between -1.0 and 1.0.
/// If the seed is none, then thread_rng will be used.
pub fn rand_points(num_points: usize, seed: Option<u64>) -> Vec<(f32, f32)> {
    let mut points = Vec::new();
    if let Some(i) = seed {
        let mut rng = ChaCha8Rng::seed_from_u64(i);
        for _ in 0..num_points {
            let x = rng.gen_range(-1.0..1.0);
            let y = rng.gen_range(-1.0..1.0);
            points.push((x, y));
        }
    } else {
        let mut rng = thread_rng();
        for _ in 0..num_points {
            let x = rng.gen_range(-1.0..1.0);
            let y = rng.gen_range(-1.0..1.0);
            points.push((x, y));
        }
    }
    points
}

pub fn rand_n_blocks(
    n_rows: usize,
    n_cols: usize,
    delta_row: f32,
    delta_col: f32,
    n: usize,
) -> Vec<Block> {
    let mut blocks = Vec::new();
    while blocks.len() < n {
        let mut new_block;
        loop {
            new_block = Block::new_rand(n_rows, n_cols, delta_row, delta_col, -1.0..1.0, -1.0..1.0);
            let overlapped = blocks.iter().any(|block| overlapping(block, &new_block));
            if !overlapped {
                blocks.push(new_block);
                break;
            }
        }
    }
    blocks
}

pub fn rand_n_blocks_as_points(
    n_rows: usize,
    n_cols: usize,
    delta_row: f32,
    delta_col: f32,
    n: usize,
) -> Vec<(f32, f32)> {
    let rand_blocks = rand_n_blocks(n_rows, n_cols, delta_row, delta_col, n);
    let rand_points;
    {
        let mut temp_points = Vec::new();
        for block in rand_blocks {
            temp_points.extend_from_slice(&block.points);
        }
        rand_points = temp_points;
    }
    rand_points
}

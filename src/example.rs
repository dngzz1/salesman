use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use salesman::block::*;

pub fn get_circle(num_points: usize) -> Vec<(f32, f32)> {
    let mut points = Vec::new();
    let pi = std::f32::consts::PI;
    for i in 0..num_points {
        let angle = 2.0 * pi * (i as f32) / num_points as f32;
        let x = angle.cos();
        let y = angle.sin();
        let p = (x, y);
        points.push(p);
    }
    points.shuffle(&mut thread_rng());
    points
}

pub fn rand_points(num_points: usize) -> Vec<(f32, f32)> {
    let mut points = Vec::new();
    for _ in 0..num_points {
        let x = rand::thread_rng().gen_range(-1.0..1.0);
        let y = rand::thread_rng().gen_range(-1.0..1.0);
        points.push((x, y));
    }
    points
}

pub fn rand_block(n_rows: usize, n_cols: usize, delta_row: f32, delta_col: f32) -> Block {
    let width = delta_row * n_rows as f32;
    let height = delta_col * n_cols as f32;
    let max_x = 1.0 - width as f32;
    let max_y = 1.0 - height as f32;
    let top_left_x = rand::thread_rng().gen_range(-1.0..max_x);
    let top_left_y = rand::thread_rng().gen_range(-1.0..max_y);
    let top_left = (top_left_x, top_left_y);
    let block = Block::new(n_rows, n_cols, delta_row, delta_col, top_left);
    block
}

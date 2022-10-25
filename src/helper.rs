use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

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

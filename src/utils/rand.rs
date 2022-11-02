use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Creates a vector of random points between -1.0 and 1.0.
/// If the seed is none, then thread_rng will be used.
pub fn get_points(num_points: usize, seed: Option<u64>) -> Vec<(f32, f32)> {
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

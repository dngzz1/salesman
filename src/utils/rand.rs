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

/// shuffles a vector using chacha if rng.is_some(), else use thread_rng()
pub fn shuffle_by_chacha_or<T>(vec: &mut [T], rng: &mut Option<ChaCha8Rng>) {
    match rng {
        Some(chacha) => vec.shuffle(chacha),
        None => vec.shuffle(&mut thread_rng()),
    }
}

pub fn chacha_rand_range(start: usize, end: usize, rng: &mut Option<ChaCha8Rng>) -> usize {
    match rng {
        Some(chacha) => chacha.gen_range(start..end),
        None => thread_rng().gen_range(start..end),
    }
}

pub fn choose_mut<'a, T>(vec: &'a mut [T], rng: &mut Option<ChaCha8Rng>) -> &'a mut T {
    let n = vec.len();
    let i = chacha_rand_range(0, n, rng);
    &mut vec[i]
}

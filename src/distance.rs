pub fn euclidean(p: (f32, f32), q: (f32, f32)) -> f32 {
    let dx = p.0 - q.0;
    let dy = p.1 - q.1;
    (dx * dx + dy * dy).sqrt()
}

pub fn portrait_euclidean(p: (f32, f32), q: (f32, f32)) -> f32 {
    let dx = (p.0 - q.0) * 1.;
    let dy = (p.1 - q.1) * 10.;
    (dx * dx + dy * dy).sqrt()
}

pub fn landscape_euclidean(p: (f32, f32), q: (f32, f32)) -> f32 {
    let dx = (p.0 - q.0) * 10.;
    let dy = (p.1 - q.1) * 1.;
    (dx * dx + dy * dy).sqrt()
}

pub fn make_distance_vec<F>(points: &[(f32, f32)], distance_fn: &F) -> Vec<f32>
where
    F: Fn((f32, f32), (f32, f32)) -> f32,
{
    let length = points.len();
    let mut distances = vec![0.0; length * length];
    for i in 0..length {
        for j in 0..length {
            distances[i * length + j] = distance_fn(points[i], points[j]);
        }
    }
    distances
}

#[cfg(test)]
mod test_distance;

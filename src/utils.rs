use ordered_float::NotNan;

pub fn loop_distance(points: &[(f32, f32)]) -> f32 {
    let mut sum = 0.0;
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        sum += crate::distance::euclidean(points[i], points[j]);
    }
    sum
}

pub fn argmin(points: Vec<f32>) -> usize {
    let non_nan_floats: Vec<_> = points
        .iter()
        .cloned()
        .map(NotNan::new) // Attempt to convert each f32 to a NotNan
        .filter_map(Result::ok) // Unwrap the `NotNan`s and filter out the `NaN` values
        .collect();

    let min = non_nan_floats.iter().min().unwrap();
    let index = non_nan_floats
        .iter()
        .position(|element| element == min)
        .unwrap();
    index
}

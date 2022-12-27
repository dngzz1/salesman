use ordered_float::NotNan;

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

// pub fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
//     let mut indices = (0..data.len()).collect::<Vec<_>>();
//     indices.sort_by_key(|&i| std::cmp::Reverse(&data[i]));
//     indices
// }

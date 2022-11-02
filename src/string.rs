/// First performs a clustering algorithm, then solves Traveling Salesman on each cluster, and finally cuts the longest segment out.
/// # Examples
/// ```
/// use salesman::string::get_string_order;
/// let points = vec![(-0.5, -0.5), (-0.6, -0.5), (-0.6, -0.4), (0.5, 0.5), (0.6, 0.5), (0.6, 0.6)];
/// let salesmen_capacities = vec![3, 3];
/// let intensity = 10.0;
/// let seed = Some(42);
/// let order = get_string_order(&points, &salesmen_capacities, intensity, seed);
/// assert!(&order[0..3] == &vec![0, 1, 2] || &order[0..3] == &vec![2, 1, 0]);
/// assert!(&order[3..6] == &vec![3, 4, 5] || &order[3..6] == &vec![5, 4, 3]);
/// ```
pub fn get_string_order(
    points: &[(f32, f32)],
    salesmen_capacities: &[usize],
    intensity: f32,
    seed: Option<u64>,
) -> Vec<usize> {
    let mut clustered_order =
        crate::cluster::cluster_order(points, salesmen_capacities, intensity, seed);
    for i in 0..salesmen_capacities.len() {
        let range_start = salesmen_capacities[0..i].iter().sum::<usize>();
        let range_end = salesmen_capacities[0..(i + 1)].iter().sum::<usize>();
        let mut filtered_points = Vec::new();
        for j in range_start..range_end {
            filtered_points.push(points[clustered_order[j]]);
        }
        let slice_order = crate::anneal::shortest_path_order(&filtered_points, 5, false, seed);
        clustered_order = crate::permute::permute_slice(
            &clustered_order,
            &(range_start..range_end).collect::<Vec<usize>>().to_vec(),
            &slice_order,
        );
    }
    clustered_order
}

pub fn get_string_order(
    points: &[(f32, f32)],
    salesmen_capacities: &[usize],
    seed: Option<u64>,
) -> Vec<usize> {
    let mut clustered_order = crate::cluster::cluster_order(&points, &salesmen_capacities, 10.0);
    for i in 0..salesmen_capacities.len() {
        let range_start = salesmen_capacities[0..i].iter().sum::<usize>();
        let range_end = salesmen_capacities[0..(i + 1)].iter().sum::<usize>();
        let mut filtered_points = Vec::new();
        for j in range_start..range_end {
            filtered_points.push(points[clustered_order[j]]);
        }
        let slice_order = crate::anneal::shortest_path_order(&filtered_points, 2, false, seed);
        clustered_order = crate::permute::permute_slice(
            &clustered_order,
            &(range_start..range_end).collect::<Vec<usize>>().to_vec(),
            &slice_order,
        );
    }
    clustered_order
}

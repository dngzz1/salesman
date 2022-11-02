fn cluster_i_metric_from_order<F>(
    points: &[(f32, f32)],
    salesmen_capacities: &[usize],
    distance_fn: &F,
    order: &[usize],
    i: usize,
) -> f32
where
    F: Fn((f32, f32), (f32, f32)) -> f32 + Clone,
{
    assert!(i < salesmen_capacities.len());
    let range_start = salesmen_capacities[0..i].iter().sum::<usize>();
    let range_end = salesmen_capacities[0..(i + 1)].iter().sum::<usize>();
    let relevant_points = order[range_start..range_end]
        .iter()
        .map(|&j| points[j])
        .collect::<Vec<_>>();
    cluster_metric(&relevant_points, distance_fn)
}

pub fn cluster_metric_from_order<F>(
    points: &[(f32, f32)],
    salesmen_capacities: &[usize],
    distance_fn: &F,
    order: &[usize],
) -> f32
where
    F: Fn((f32, f32), (f32, f32)) -> f32 + Clone,
{
    let mut result = 0.0;
    for i in 0..salesmen_capacities.len() {
        result += cluster_i_metric_from_order(points, salesmen_capacities, distance_fn, order, i)
    }
    result
}

fn barycenter(points: &[(f32, f32)]) -> (f32, f32) {
    let n = points.len() as f32;
    let x = points.iter().map(|&point| point.0).sum::<f32>();
    let y = points.iter().map(|&point| point.1).sum::<f32>();
    (x / n, y / n)
}

fn cluster_metric<F>(points: &[(f32, f32)], distance_fn: &F) -> f32
where
    F: Fn((f32, f32), (f32, f32)) -> f32 + Clone,
{
    let b = barycenter(points);
    let mut result = 0.0;
    for &point in points {
        result += distance_fn(point, b);
    }
    result
}

pub fn get_salesman_index(i: usize, salesmen_capacities: &[usize]) -> usize {
    let mut index = 0;
    while salesmen_capacities[0..(index + 1)].iter().sum::<usize>() - 1 < i {
        index += 1;
    }
    index
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_barycenter() {
        let points = vec![(1.0, 2.0), (3.0, 0.0)];
        let barycenter = barycenter(&points);
        assert_eq!(barycenter, (2.0, 1.0));
    }

    #[test]
    fn test_cluster_i_metric_from_order() {
        let points = (0..10)
            .collect::<Vec<_>>()
            .iter()
            .map(|&x| (x as f32, 0.0))
            .collect::<Vec<(f32, f32)>>();
        let order = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let salesmen_capacities = vec![2, 3, 5];
        let distance_fn = crate::utils::distance::euclidean;
        assert_eq!(
            cluster_i_metric_from_order(&points, &salesmen_capacities, &distance_fn, &order, 0),
            1.0
        );
        assert_eq!(
            cluster_i_metric_from_order(&points, &salesmen_capacities, &distance_fn, &order, 1),
            2.0
        );
        assert_eq!(
            cluster_i_metric_from_order(&points, &salesmen_capacities, &distance_fn, &order, 2),
            6.0
        );
        assert_eq!(
            cluster_metric_from_order(&points, &salesmen_capacities, &distance_fn, &order),
            9.0
        );
    }

    #[test]
    fn test_get_salesman_index() {
        let salesmen_capacities = vec![10, 5, 5];
        assert_eq!(get_salesman_index(0, &salesmen_capacities), 0);
        assert_eq!(get_salesman_index(1, &salesmen_capacities), 0);
        assert_eq!(get_salesman_index(9, &salesmen_capacities), 0);
        assert_eq!(get_salesman_index(10, &salesmen_capacities), 1);
        assert_eq!(get_salesman_index(14, &salesmen_capacities), 1);
        assert_eq!(get_salesman_index(15, &salesmen_capacities), 2);
        assert_eq!(get_salesman_index(16, &salesmen_capacities), 2);
        assert_eq!(get_salesman_index(19, &salesmen_capacities), 2);
    }
}

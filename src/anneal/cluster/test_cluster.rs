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
    let distance_fn = crate::distance::euclidean;
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

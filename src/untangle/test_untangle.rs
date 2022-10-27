use super::*;

#[test]
fn test_get_untangled_order() {
    let points = vec![(0., 0.), (1., 0.), (2., 0.), (2., 1.), (1., 1.), (0., 1.)];
    let order = vec![0, 4, 2, 3, 1, 5];
    let new_order = get_untangled_order(&points, &order);
    assert_eq!(new_order, vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_disconnect_longest_string() {
    let points = vec![
        (0., 0.),
        (0.5, 0.),
        (1., 0.0),
        (1., 1.0),
        (0.5, 1.),
        (0., 2.),
    ];
    let order = vec![1, 2, 3, 4, 5, 0];
    let new_order = disconnect_longest_string(&points, &order);
    assert_eq!(new_order, vec![0, 1, 2, 3, 4, 5]);
}

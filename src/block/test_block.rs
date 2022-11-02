use super::*;
#[test]
fn can_get_coord_from_pos() {
    let pos_top_left = (10.0, 100.0);
    let n_cols = 3;
    let n_rows = 2;
    let delta_col = 0.1;
    let delta_row = 0.5;
    let shape = vec![false, true, true, true, false, true];
    let block = Block::new_from_bool(pos_top_left, n_cols, n_rows, delta_col, delta_row, shape);
    assert_eq!(block.data[1], Some((10.5, 100.0)));
    assert_eq!(block.data[2], Some((11.0, 100.0)));
    assert_eq!(block.data[3], Some((10.0, 100.1)));
    assert_eq!(block.data[5], Some((11.0, 100.1)));
}

#[test]
fn can_b_coord_to_shape() {
    let b_coords = vec![(0, 1), (0, 2), (1, 0), (1, 2)];
    let n_cols = 3;
    let n_rows = 2;
    let shape = b_coord_to_shape(n_cols, n_rows, b_coords);
    assert_eq!(shape, vec![false, true, true, true, false, true]);

    let pos_top_left = (10.0, 100.0);
    let n_cols = 3;
    let n_rows = 2;
    let delta_col = 0.1;
    let delta_row = 0.5;
    let b_coords = vec![(0, 1), (0, 2), (1, 0), (1, 2)];
    let block =
        Block::new_from_b_coords(pos_top_left, n_cols, n_rows, delta_col, delta_row, b_coords);
    assert_eq!(block.data[1], Some((10.5, 100.0)));
    assert_eq!(block.data[2], Some((11.0, 100.0)));
    assert_eq!(block.data[3], Some((10.0, 100.1)));
    assert_eq!(block.data[5], Some((11.0, 100.1)));
}

#[test]
fn can_get_neighbor_indices() {
    // let shape = vec![true, true, true, true, true, true, true, true, false];
    let nx = 3;
    let ny = 3;
    assert_eq!(get_neighbor_indices(nx, ny, 0), vec![1, 3]);
    assert_eq!(get_neighbor_indices(nx, ny, 1), vec![0, 2, 4]);
    assert_eq!(get_neighbor_indices(nx, ny, 2), vec![1, 5]);
    assert_eq!(get_neighbor_indices(nx, ny, 3), vec![0, 4, 6]);
    assert_eq!(get_neighbor_indices(nx, ny, 4), vec![1, 3, 5, 7]);
    assert_eq!(get_neighbor_indices(nx, ny, 5), vec![2, 4, 8]);
    assert_eq!(get_neighbor_indices(nx, ny, 6), vec![3, 7]);
    assert_eq!(get_neighbor_indices(nx, ny, 7), vec![4, 6, 8]);
    assert_eq!(get_neighbor_indices(nx, ny, 8), vec![5, 7]);
}

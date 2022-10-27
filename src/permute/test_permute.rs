use super::*;

#[test]
fn can_permute_slice() {
    let main_order = vec![0, 1, 2, 3, 4, 5, 6];
    let subslice_indices = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(
        permute_slice(&main_order, &subslice_indices, &[1, 2, 3, 4, 5, 0]),
        vec![0, 2, 3, 4, 5, 6, 1]
    );
    assert_eq!(
        permute_slice(&main_order, &subslice_indices, &[1, 0, 2, 3, 4, 5]),
        vec![0, 2, 1, 3, 4, 5, 6]
    );
    assert_eq!(
        permute_slice(&main_order, &[0, 2, 3, 4, 5, 6], &[1, 0, 2, 3, 4, 5]),
        vec![2, 1, 0, 3, 4, 5, 6]
    );
}

#[test]
fn can_rotate_slice() {
    let main_order = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(
        permute_slice(&main_order, &[0, 1, 2], &[1, 2, 0]),
        vec![1, 2, 0, 3, 4, 5, 6, 7, 8]
    );
    assert_eq!(
        permute_slice(&main_order, &[3, 4, 5], &[1, 2, 0]),
        vec![0, 1, 2, 4, 5, 3, 6, 7, 8]
    );
    assert_eq!(
        permute_slice(&main_order, &[6, 7, 8], &[1, 2, 0]),
        vec![0, 1, 2, 3, 4, 5, 7, 8, 6]
    );
}

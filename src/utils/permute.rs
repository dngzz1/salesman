pub fn permute_slice(
    main_order: &[usize],
    subslice_indices: &[usize],
    slice_order: &[usize],
) -> Vec<usize> {
    assert_eq!(subslice_indices.len(), slice_order.len());
    assert!(subslice_indices.len() <= main_order.len());
    let mut result = main_order.to_vec();
    for i in 0..slice_order.len() {
        result[subslice_indices[i]] = main_order[subslice_indices[slice_order[i]]];
    }
    result
}

// pub fn permute_rows(vec: &[usize], block_length: usize, order: &[usize]) -> Vec<usize> {
//     let mut new_vec = vec![0; vec.len()];
//     let n_blocks = vec.len() / block_length;
//     for i in 0..n_blocks {
//         for j in 0..block_length {
//             new_vec[i * block_length + j] = vec[order[i] * block_length + j];
//         }
//     }
//     new_vec
// }

#[cfg(test)]
mod test {
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

    // #[test]
    // fn can_permute_rows() {
    //     let vec = vec![0, 1, 10, 11, 20, 21];
    //     let block_length = 2;
    //     let order = vec![1, 0, 2];
    //     let new_vec = permute_rows(&vec, block_length, &order);
    //     assert_eq!(new_vec, vec![10, 11, 0, 1, 20, 21]);
    // }
}

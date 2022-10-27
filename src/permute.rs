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

#[cfg(test)]
mod test_permute;

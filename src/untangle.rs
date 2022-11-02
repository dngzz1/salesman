use crate::intersection;

pub fn get_untangled_order(points: &[(f32, f32)], order: &[usize]) -> Vec<usize> {
    let mut new_order = order.to_vec();
    const TIMEOUT: usize = 1000;
    let n = points.len();
    let num_segments = points.len();
    let mut loop_count = 0;
    loop {
        if loop_count > TIMEOUT {
            return new_order;
        }
        let mut i = 0;
        let mut j = 0;
        'loop_i: while i < num_segments {
            j = (i + 2) % n;
            while j < num_segments {
                let a = points[new_order[i]];
                let b = points[new_order[(i + 1) % n]];
                let c = points[new_order[j]];
                let d = points[new_order[(j + 1) % n]];
                if intersection::intersects(a, b, c, d) {
                    new_order.swap((i + 1) % n, j);
                    loop_count += 1;
                    break 'loop_i;
                }
                j += 1;
            }
            i += 1;
        }
        if i == num_segments && j == num_segments {
            return new_order;
        }
    }
}

pub fn disconnect_longest_string(points: &[(f32, f32)], order: &[usize]) -> Vec<usize> {
    let mut index_of_longest_string = 0;
    let mut length_of_longest_string = 0.;
    let n = points.len();
    for i in 0..n {
        let p = points[order[i]];
        let q = points[order[(i + 1) % n]];
        let distance = crate::distance::euclidean(p, q);
        if distance > length_of_longest_string {
            index_of_longest_string = i;
            length_of_longest_string = distance;
        }
    }
    let mut new_order = Vec::new();
    for &item in order.iter().take(n).skip(index_of_longest_string + 1) {
        new_order.push(item);
    }
    for &item in order.iter().take(index_of_longest_string + 1) {
        new_order.push(item);
    }
    new_order
}

#[cfg(test)]
mod test_untangle;

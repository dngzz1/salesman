use crate::intersection;

pub fn get_untangled_order(points: &[(f32, f32)], order: &[usize]) -> Vec<usize> {
    let mut new_order = order.to_vec();
    let n = points.len();
    let num_segments = points.len();
    loop {
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

#[cfg(test)]
mod test_untangle;

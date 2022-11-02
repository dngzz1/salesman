// #[derive(Debug, Clone)]
pub struct Block {
    pub pos_top_left: (f32, f32),
    pub nx: usize,
    pub ny: usize,
    pub dx: f32,
    pub dy: f32,
    pub shape: Vec<bool>,
    data: Vec<Option<(f32, f32)>>,
    pub n_points: usize,
}

impl Block {
    pub fn new_from_bool(
        pos_top_left: (f32, f32),
        nx: usize,
        ny: usize,
        dx: f32,
        dy: f32,
        shape: Vec<bool>,
    ) -> Self {
        assert!(shape.len() / nx == ny);
        let n_points = shape.iter().filter(|&x| *x).count();
        let mut data = Vec::new();
        for j in 0..ny {
            for i in 0..nx {
                let index = j * nx + i;
                if shape[index] {
                    let x = pos_top_left.0 + dy * i as f32;
                    let y = pos_top_left.1 + dx * j as f32;
                    data.push(Some((x, y)));
                } else {
                    data.push(None);
                }
            }
        }
        Self {
            pos_top_left,
            nx,
            ny,
            dx,
            dy,
            shape,
            data,
            n_points,
        }
    }
    pub fn new_from_b_coords(
        pos_top_left: (f32, f32),
        n_cols: usize,
        n_rows: usize,
        delta_row: f32,
        delta_col: f32,
        b_coords: Vec<(usize, usize)>,
    ) -> Self {
        let shape = b_coord_to_shape(n_cols, n_rows, b_coords);
        Block::new_from_bool(pos_top_left, n_cols, n_rows, delta_row, delta_col, shape)
    }

    pub fn get_points(&self, flip_y: bool) -> Vec<(f32, f32)> {
        self.data
            .iter()
            .filter(|&&p| p.is_some())
            .map(|&p| p.unwrap())
            .map(|p| match flip_y {
                true => (p.0, -p.1),
                false => p,
            })
            .collect::<Vec<_>>()
    }
}

#[allow(dead_code)]
fn b_coord_to_shape(n_cols: usize, n_rows: usize, b_coords: Vec<(usize, usize)>) -> Vec<bool> {
    let mut shape = vec![false; n_cols * n_rows];
    for coord in b_coords {
        let (i, j) = coord;
        let index = i * n_cols + j;
        shape[index] = true;
    }
    shape
}

#[allow(dead_code)]
fn get_b_coord_from_index(nx: usize, ny: usize, index: usize) -> (usize, usize) {
    (index % nx, index / ny)
}

#[allow(dead_code)]
fn get_neighbor_indices(nx: usize, ny: usize, index: usize) -> Vec<usize> {
    let mut neighbor_indices = Vec::new();
    if index >= nx {
        neighbor_indices.push(index - nx);
    }
    if index % nx != 0 {
        neighbor_indices.push(index - 1);
    }
    if index % nx != nx - 1 {
        neighbor_indices.push(index + 1);
    }
    if index + nx < nx * ny {
        neighbor_indices.push(index + nx);
    }
    neighbor_indices
}

#[cfg(test)]
mod test_block;

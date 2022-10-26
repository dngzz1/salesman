pub struct Block {
    pub n_rows: usize,
    pub n_cols: usize,
    pub delta_row: f32,
    pub delta_col: f32,
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub width: f32,
    pub height: f32,
    pub pos_top_left: (f32, f32),
    pub pos_center: (f32, f32),
    pub points: Vec<(f32, f32)>,
}

impl Block {
    pub fn new(
        n_rows: usize,
        n_cols: usize,
        delta_row: f32,
        delta_col: f32,
        pos_top_left: (f32, f32),
    ) -> Self {
        let width = delta_row * n_rows as f32;
        let height = delta_col * n_cols as f32;
        let min_x = pos_top_left.0;
        let min_y = pos_top_left.1;
        let max_x = pos_top_left.0 + width;
        let max_y = pos_top_left.1 + height;
        let pos_center = (min_x + width / 2.0, min_y + height / 2.0);
        let mut points = Vec::new();
        for i in 0..n_rows {
            for j in 0..n_cols {
                let x = min_x + i as f32 * delta_row;
                let y = min_y + j as f32 * delta_col;
                points.push((x, y));
            }
        }
        Self {
            n_rows,
            n_cols,
            delta_row,
            delta_col,
            min_x,
            min_y,
            max_x,
            max_y,
            width,
            height,
            pos_top_left,
            pos_center,
            points,
        }
    }
}

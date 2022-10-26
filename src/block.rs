use std::ops::Range;

use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
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
    pub pos_center: (f32, f32),
    pub points: Vec<(f32, f32)>,
}

impl Block {
    pub fn new(
        n_rows: usize,
        n_cols: usize,
        delta_row: f32,
        delta_col: f32,
        pos_center: (f32, f32),
    ) -> Self {
        let width = delta_row * n_rows as f32;
        let height = delta_col * n_cols as f32;
        let min_x = pos_center.0 - width / 2.0;
        let min_y = pos_center.1 - height / 2.0;
        let max_x = pos_center.0 + width / 2.0;
        let max_y = pos_center.1 + height / 2.0;
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
            pos_center,
            points,
        }
    }

    pub fn new_rand(
        n_rows: usize,
        n_cols: usize,
        delta_row: f32,
        delta_col: f32,
        x_range: Range<f32>,
        y_range: Range<f32>,
    ) -> Block {
        let width = delta_row * n_rows as f32;
        let height = delta_col * n_cols as f32;
        let x_min = x_range.start + width / 2.0;
        let x_max = x_range.end - width / 2.0;
        let y_min = y_range.start + height / 2.0;
        let y_max = y_range.end - height / 2.0;
        let center_x = rand::thread_rng().gen_range(x_min..x_max);
        let center_y = rand::thread_rng().gen_range(y_min..y_max);
        let pos_center = (center_x, center_y);
        Block::new(n_rows, n_cols, delta_row, delta_col, pos_center)
    }
}

pub fn overlapping(block0: &Block, block1: &Block) -> bool {
    let min_x = block1.min_x;
    let min_y = block1.min_y;
    let max_x = block1.max_x;
    let max_y = block1.max_y;
    let p = (block0.min_x, block0.min_y);
    let q = (block0.max_x, block0.max_y);
    let r = (block0.min_x, block0.max_y);
    let s = (block0.max_x, block0.min_y);
    min_x <= p.0 && p.0 <= max_x && min_y <= p.1 && p.1 <= max_y
        || min_x <= q.0 && q.0 <= max_x && min_y <= q.1 && q.1 <= max_y
        || min_x <= r.0 && r.0 <= max_x && min_y <= r.1 && r.1 <= max_y
        || min_x <= s.0 && s.0 <= max_x && min_y <= s.1 && s.1 <= max_y
}

#[cfg(test)]
mod test_block;

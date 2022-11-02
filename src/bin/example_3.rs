#[allow(unused_imports)]
use rand::prelude::*;
use salesman::block::Block;

fn main() {
    let pos_top_left = (-0.5, -0.9);
    let n_cols = 6;
    let n_rows = 6;
    let delta_col = 0.1;
    let delta_row = 0.1;
    let mut shape = vec![true; n_cols * n_rows];
    shape[15] = false;
    let block = Block::new_from_bool(pos_top_left, n_cols, n_rows, delta_col, delta_row, shape);
    let points = block.get_points(true);
    let distance_fn = salesman::distance::portrait_euclidean;
    let is_loop = false;

    let salesmen_capacities = vec![12, 6, 6, 6, 5];
    let intensity = 10.0;
    let seed = None;
    let order = salesman::string::get_string_order(
        &points,
        &salesmen_capacities,
        &distance_fn,
        is_loop,
        intensity,
        seed,
    );
    let is_loop = false;
    let display_string = true;
    let filename = "example_3/block";
    let caption = "Block";
    salesman::plot::plot_strings(
        &points,
        &order,
        &salesmen_capacities,
        is_loop,
        display_string,
        filename,
        caption,
    );
}

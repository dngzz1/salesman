use salesman::block::Block;
use std::fs;

fn main() {
    let filename = "resources/blocks/block01.txt";
    draw_block(filename);
}

fn draw_block(filename: &str) {
    let (nx, ny, shape) = get_shape(filename);
    let pos_top_left = (-0.9, -0.9);
    let delta_col = 0.1;
    let delta_row = 0.1;
    let block = Block::new_from_bool(pos_top_left, nx, ny, delta_col, delta_row, shape);
    let points = block.get_points(true);
    let distance_fn = salesman::utils::distance::landscape_euclidean;
    let is_loop = false;

    let salesmen_capacities = {
        let salesman_capacity = 7;
        let n_strings = block.n_points / salesman_capacity;
        let leftover = block.n_points % salesman_capacity;

        match leftover {
            0 => vec![salesman_capacity; n_strings],
            _ => {
                let mut tmp = vec![salesman_capacity; n_strings];
                tmp.push(leftover);
                tmp
            }
        }
    };
    println!("{} points, {:?}", block.n_points, salesmen_capacities);
    let intensity = 12.0;
    let seed = None;
    let order = salesman::string::get_string_order(
        &points,
        &salesmen_capacities,
        &distance_fn,
        is_loop,
        intensity,
        seed,
    );
    let display_string = true;
    let filename = "example_4/block";
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

fn get_shape(filename: &str) -> (usize, usize, Vec<bool>) {
    let input = fs::read_to_string(filename).unwrap();
    let mut shape = vec![];
    let mut nx = 0;
    let mut ny = 0;
    for line in input.lines() {
        nx = line.len();
        for c in line.chars() {
            match c {
                '0' => shape.push(false),
                '1' => shape.push(true),
                _ => {}
            }
        }
        ny += 1;
    }
    (nx, ny, shape)
}

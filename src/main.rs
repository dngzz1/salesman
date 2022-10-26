pub mod example;
pub mod plot;
use example::*;
use salesman::anneal::*;

fn main() {
    // Solve travelling salesman.
    let rand_blocks = rand_n_blocks(5, 7, 0.16, 0.08, 1);
    let rand_points;
    {
        let mut temp_points = Vec::new();
        for block in rand_blocks {
            temp_points.extend_from_slice(&block.points);
        }
        rand_points = temp_points;
    }

    let points = shortest_path(&rand_points, 1);
    plot::plot(&points);
}

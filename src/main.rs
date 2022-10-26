pub mod plot;

fn main() {
    // Solve travelling salesman.
    let rand_points = salesman::example::rand_points_from_chacha(40);

    let points = salesman::anneal::shortest_path(&rand_points, 1);
    plot::plot(&points);
}

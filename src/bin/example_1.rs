use salesman::plot::plot_strings;
fn main() {
    let num_points = 60;
    let seed = Some(42);
    let intensity = 10.0;
    let points = salesman::utils::rand::get_points(num_points, seed);
    let distance_fn = salesman::utils::distance::euclidean;
    let is_loop = false;
    let salesmen_capacities = [num_points / 6; 6];
    let order = salesman::string::get_string_order(
        &points,
        &salesmen_capacities,
        &distance_fn,
        is_loop,
        intensity,
        seed,
    );
    plot_strings(
        &points,
        &order,
        &salesmen_capacities,
        true,
        false,
        "example_1/clusters",
        "Clusters",
    );
    plot_strings(
        &points,
        &order,
        &salesmen_capacities,
        true,
        true,
        "example_1/closed_strings",
        "Closed Strings",
    );
    plot_strings(
        &points,
        &order,
        &salesmen_capacities,
        false,
        true,
        "example_1/open_strings",
        "Open Strings",
    );
}

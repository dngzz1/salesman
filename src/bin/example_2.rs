use salesman::plot::plot_strings;
fn main() {
    let num_points = 60;
    let seed = Some(42);
    let intensity = 10.0;
    let points = salesman::example::rand_points(num_points, seed);
    let distance_fn = salesman::distance::euclidean;
    let is_loop = false;

    let salesmen_capacities = [12, 12, 10, 8, 8, 6, 4];
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
        "example_2/clusters",
        "Clusters",
    );
    plot_strings(
        &points,
        &order,
        &salesmen_capacities,
        true,
        true,
        "example_2/closed_strings",
        "Closed Strings",
    );
    plot_strings(
        &points,
        &order,
        &salesmen_capacities,
        false,
        true,
        "example_2/open_strings",
        "Open Strings",
    );
}

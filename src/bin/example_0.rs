use salesman::plot::plot_strings;
fn main() {
    let seed = Some(42);
    let intensity = 10.0;
    let points = vec![
        (-0.5, -0.5),
        (-0.6, -0.5),
        (-0.6, -0.4),
        (0.5, 0.5),
        (0.6, 0.5),
        (0.6, 0.6),
    ];
    let salesmen_capacities = vec![3, 3];
    let distance_fn = salesman::distance::euclidean;
    let order = salesman::string::get_string_order(
        &points,
        &salesmen_capacities,
        &distance_fn,
        intensity,
        seed,
    );
    plot_strings(
        &points,
        &order,
        &salesmen_capacities,
        true,
        false,
        "example_0/clusters",
        "Clusters",
    );
    plot_strings(
        &points,
        &order,
        &salesmen_capacities,
        true,
        true,
        "example_0/closed_strings",
        "Closed Strings",
    );
    plot_strings(
        &points,
        &order,
        &salesmen_capacities,
        false,
        true,
        "example_0/open_strings",
        "Open Strings",
    );
}

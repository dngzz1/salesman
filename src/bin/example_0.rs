use salesman::plot::plot_strings;
fn main() {
    let seed = Some(42);
    let intensity = 10.0;
    let points = vec![(0.1, -0.1), (0.5, 0.25), (0., 0.32), (0.3, 0.1)];
    let salesmen_capacities = vec![4];
    let order = salesman::string::get_string_order(&points, &salesmen_capacities, intensity, seed);
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

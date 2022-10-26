fn main() {
    let rand_points = salesman::example::rand_points_from_chacha(80);
    let salesmen_capacities = vec![40, 20, 10, 10];
    let points = salesman::anneal::shortest_path(&rand_points, 1);
    salesman::plot::plot_clusters(&points, &salesmen_capacities, "clustering");
}

fn main() {
    let num_points = 180;
    let rand_points = salesman::example::rand_points_from_chacha(num_points);
    let salesmen_capacities = [num_points / 6; 6];
    let points = salesman::anneal::shortest_path(&rand_points, 1);
    salesman::plot::plot_clusters(&points, &salesmen_capacities, "clustering");
}

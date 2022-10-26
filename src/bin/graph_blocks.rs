fn main() {
    let rand_points = salesman::example::rand_n_blocks_as_points(5, 4, 0.07, 0.1, 5);
    let salesmen_capacities = vec![20, 20, 20, 20, 20];
    let points = salesman::anneal::shortest_path(&rand_points, 1);
    salesman::plot::plot_clusters(&points, &salesmen_capacities, "blocks");
}

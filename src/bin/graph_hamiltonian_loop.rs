fn main() {
    let rand_points = salesman::example::rand_points_from_chacha(40);
    let points = salesman::anneal::shortest_path(&rand_points, 1);
    salesman::plot::plot_hamiltonian_loop(&points);
}

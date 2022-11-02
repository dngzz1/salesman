use salesman::genetic::cluster::*;
use salesman::plot::plot_strings;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let num_points = 100;
    let salesman_capacity = 20;
    let salesmen_capacities = vec![salesman_capacity; num_points / salesman_capacity];
    let seed = None;
    let points = salesman::utils::rand::get_points(num_points, Some(42));
    let distance_fn = salesman::utils::distance::euclidean;
    let gene_parameters = GeneParameters {
        n_individuals: 10,
        max_individuals: 200,
        max_rounds: 1000,
    };
    let mut genetic_manager = GeneticManager::new(
        &points,
        &salesmen_capacities,
        &distance_fn,
        seed,
        &gene_parameters,
    );

    genetic_manager.run_by_swap();
    let order = genetic_manager.get_fittest();
    plot_strings(
        &points,
        &order,
        &salesmen_capacities,
        true,
        false,
        "genetic/clusters",
        "Clusters",
    );
}

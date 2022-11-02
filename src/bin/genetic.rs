use salesman::genetic::cluster::GeneticManager;
use salesman::genetic::cluster::*;
use salesman::plot::plot_strings;

fn main() {
    let num_points = 60;
    let salesmen_capacities = [12, 12, 10, 8, 8, 6, 4];
    let seed = None;
    let points = salesman::utils::rand::get_points(num_points, seed);
    let distance_fn = salesman::utils::distance::euclidean;
    let gene_parameters = GeneParameters {
        n_individuals: 5,
        max_individuals: 100,
    };
    let mut genetic_manager = GeneticManager::new(
        &points,
        &salesmen_capacities,
        &distance_fn,
        seed,
        &gene_parameters,
    );
    for _ in 0..100 {
        genetic_manager.step();
        println!("{}", genetic_manager);
    }
    let order = genetic_manager.get_fittest();
    plot_strings(
        &points,
        order,
        &salesmen_capacities,
        true,
        false,
        "genetic/clusters",
        "Clusters",
    );
}

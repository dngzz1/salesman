use salesman::genetic::cluster::*;
use salesman::plot::plot_strings;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let num_points = 100;
    let salesmen_capacities = [20; 5];
    let seed = None;
    let points = salesman::utils::rand::get_points(num_points, seed);
    let distance_fn = salesman::utils::distance::euclidean;
    let gene_parameters = GeneParameters {
        n_individuals: 10,
        max_individuals: 100,
    };
    let mut genetic_manager = GeneticManager::new(
        &points,
        &salesmen_capacities,
        &distance_fn,
        seed,
        &gene_parameters,
    );
    let fitness_order = genetic_manager.get_fitness_order();
    println!("{:?}", fitness_order);
    genetic_manager.sort_fitness();

    for _ in 0..200 {
        //     for i in 0..gene_parameters.n_individuals {
        //         println!(
        //             "Individual {}, fitness: {:.4}",
        //             i,
        //             genetic_manager.get_fitness_of_ith_individual(i)
        //         )
        //     }
        genetic_manager.step();
    }
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

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
// https://medium.com/@romanboegli/solving-tsp-with-help-of-genetic-algorithm-in-java-fa2aa4349e8f
#[derive(Clone)]
pub struct GeneParameters {
    pub n_individuals: usize,
    pub max_individuals: usize,
}

pub struct GeneticManager<F>
where
    F: Fn((f32, f32), (f32, f32)) -> f32 + Clone,
{
    data: Vec<usize>,
    n_individuals: usize,
    points: Vec<(f32, f32)>,
    n_points: usize,
    salesmen_capacities: Vec<usize>,
    distance_fn: F,
    seed: Option<u64>,
    gene_parameters: GeneParameters,
    rng: Option<ChaCha8Rng>,
}

impl<F> GeneticManager<F>
where
    F: Fn((f32, f32), (f32, f32)) -> f32 + Clone,
{
    pub fn new(
        points: &[(f32, f32)],
        salesmen_capacities: &[usize],
        distance_fn: &F,
        seed: Option<u64>,
        gene_parameters: &GeneParameters,
    ) -> Self {
        let points = points.to_vec();
        let n_points = points.len();
        let n_individuals = gene_parameters.n_individuals;
        let salesmen_capacities = salesmen_capacities.to_vec();
        let distance_fn = distance_fn.clone();
        let gene_parameters = gene_parameters.clone();
        let mut rng = seed.map(ChaCha8Rng::seed_from_u64);
        let ni = gene_parameters.n_individuals;
        let mut individuals = vec![0; n_points * n_individuals];
        for i in 0..ni {
            let mut order = (0..n_points).collect::<Vec<usize>>();
            crate::utils::rand::shuffle_by_chacha_or(&mut order, &mut rng);
            for j in 0..n_points {
                individuals[i * n_points + j] = order[j];
            }
        }
        Self {
            data: individuals,
            n_individuals,
            points,
            n_points,
            salesmen_capacities,
            distance_fn,
            seed,
            gene_parameters,
            rng,
        }
    }
    pub fn get_fittest(&self) -> Vec<usize> {
        let n_points = self.points.len();
        self.data[0..n_points].to_vec()
    }
    pub fn get_fitness_of_ith_individual(&self, i: usize) -> f32 {
        let order = get_ith_block(i, &self.data, self.n_points);
        get_fitness(
            &self.points,
            &self.salesmen_capacities,
            &self.distance_fn,
            order,
        )
    }
    pub fn get_fitness_order(&self) -> Vec<usize> {
        let fitness_vec = (0..(self.data.len() / self.n_points))
            .map(|i| {
                get_fitness_of_ith_individual(
                    i,
                    &self.data,
                    self.n_points,
                    &self.points,
                    &self.salesmen_capacities,
                    &self.distance_fn,
                )
            })
            .map(|i| (i * 10000.0).round() as i32)
            .collect::<Vec<_>>();
        crate::utils::math::argsort(&fitness_vec)
    }
    pub fn sort_fitness(&mut self) {
        let fitness_order = self.get_fitness_order();
        self.data = crate::utils::permute::permute_rows(&self.data, self.n_points, &fitness_order)
    }

    pub fn step(&mut self) {
        let mut new_individuals = self.data.clone();
        while new_individuals.len() < self.gene_parameters.max_individuals * self.n_points {
            let individual_index = crate::utils::rand::chacha_rand_range(
                0,
                self.data.len() / self.n_points,
                &mut self.rng,
            );
            let mut individual =
                get_ith_block(individual_index, &self.data, self.n_points).to_vec();
            let i = crate::utils::rand::chacha_rand_range(0, self.n_points, &mut self.rng);
            let j = crate::utils::rand::chacha_rand_range(0, self.n_points, &mut self.rng);
            individual.swap(i, j);
            new_individuals.append(&mut individual);
        }
        self.data = new_individuals;
        self.sort_fitness();
        self.data.truncate(self.n_individuals * self.n_points);
    }
}

fn get_ith_block(i: usize, vec: &[usize], block_size: usize) -> &[usize] {
    &vec[(i * block_size)..((i + 1) * block_size)]
}

fn get_fitness<F>(
    points: &[(f32, f32)],
    salesmen_capacities: &[usize],
    distance_fn: &F,
    order: &[usize],
) -> f32
where
    F: Fn((f32, f32), (f32, f32)) -> f32 + Clone,
{
    -crate::utils::cluster::cluster_metric_from_order(
        points,
        salesmen_capacities,
        &distance_fn,
        &order,
    )
}

pub fn get_fitness_of_ith_individual<F>(
    i: usize,
    individuals: &[usize],
    block_size: usize,
    points: &[(f32, f32)],
    salesmen_capacities: &[usize],
    distance_fn: &F,
) -> f32
where
    F: Fn((f32, f32), (f32, f32)) -> f32 + Clone,
{
    let order = get_ith_block(i, individuals, block_size);
    get_fitness(points, salesmen_capacities, distance_fn, order)
}

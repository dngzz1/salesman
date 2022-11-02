use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
// https://medium.com/@romanboegli/solving-tsp-with-help-of-genetic-algorithm-in-java-fa2aa4349e8f
#[derive(Clone)]
pub struct GeneParameters {
    pub n_individuals: usize,
    pub max_individuals: usize,
    pub max_rounds: usize,
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
            gene_parameters,
            rng,
        }
    }
    pub fn get_fittest(&self) -> Vec<usize> {
        self.data[0..self.n_points].to_vec()
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

    fn get_fitness_vec(&self) -> Vec<i32> {
        (0..(self.data.len() / self.n_points))
            .map(|i| self.get_fitness_of_ith_individual(i))
            .map(|i| (i * 10000.0).round() as i32)
            .collect::<Vec<_>>()
    }
    pub fn get_fitness_order(&self) -> Vec<usize> {
        crate::utils::math::argsort(&self.get_fitness_vec())
    }
    fn sort_fitness(&mut self) {
        let fitness_order = self.get_fitness_order();
        self.data = crate::utils::permute::permute_rows(&self.data, self.n_points, &fitness_order)
    }

    pub fn step_by_crossover(&mut self) {
        let mut new_individuals = self.data.clone();
        while new_individuals.len() < self.gene_parameters.max_individuals * self.n_points {
            let (i0, i1) = {
                let mut f = || {
                    crate::utils::rand::chacha_rand_range(
                        0,
                        self.data.len() / self.n_points,
                        &mut self.rng,
                    )
                };
                (f(), f())
            };
            let order0 = get_ith_block(i0, &self.data, self.n_points);
            let order1 = get_ith_block(i1, &self.data, self.n_points);
            let m0 = crate::utils::rand::chacha_rand_range(0, self.n_points, &mut self.rng);
            let m1 = crate::utils::rand::chacha_rand_range(0, self.n_points, &mut self.rng);
            let start = std::cmp::min(m0, m1);
            let end = std::cmp::max(m0, m1);
            let mut child = crossover(order0, order1, start, end);
            mutate(&mut child, &self.salesmen_capacities, &mut self.rng);
            new_individuals.append(&mut child);
        }
        self.data = new_individuals;
        self.sort_fitness();
        self.data.truncate(self.n_individuals * self.n_points);
    }

    pub fn step_by_swap(&mut self) {
        let mut new_individuals = self.data.clone();
        while new_individuals.len() < self.gene_parameters.max_individuals * self.n_points {
            let individual_index = crate::utils::rand::chacha_rand_range(
                0,
                self.data.len() / self.n_points,
                &mut self.rng,
            );
            let mut child = get_ith_block(individual_index, &self.data, self.n_points).to_vec();
            mutate(&mut child, &self.salesmen_capacities, &mut self.rng);
            new_individuals.append(&mut child);
        }
        self.data = new_individuals;
        self.sort_fitness();
        self.data.truncate(self.n_individuals * self.n_points);
    }

    pub fn converged(&self) -> bool {
        let arr = self.get_fitness_vec();
        let first = arr[0];
        arr.iter().all(|&item| item == first)
    }

    pub fn run_by_swap(&mut self) {
        for round in 0..self.gene_parameters.max_rounds {
            println!("Round {}:", round);
            for i in 0..self.n_individuals {
                println!(
                    "Individual {}, fitness: {:.4}",
                    i,
                    self.get_fitness_of_ith_individual(i)
                )
            }
            if self.converged() {
                println!("{:?}", self.get_fitness_vec());
                break;
            }
            self.step_by_swap();
        }
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
        order,
    )
}

fn crossover(order0: &[usize], order1: &[usize], start: usize, end: usize) -> Vec<usize> {
    let order0 = order0.to_vec();
    let order1 = order1.to_vec();
    let length = order0.len();
    let mut child = vec![length + 1; length]; // use length + 1 as placeholder
                                              // fill in genome of first parent
    child[start..end].copy_from_slice(&order0[start..end]);
    // fill in genome of second parent
    let mut j = 0;
    for i in 0..length {
        if child[i] == length + 1 {
            // fillable slot found
            let mut l_exit = false;
            while !l_exit {
                // check if vertex-index already included
                if child.contains(&order1[j]) {
                    j += 1;
                } else {
                    l_exit = true;
                }
            }
            child[i] = order1[j];
        }
    }
    child
}

fn mutate<T>(vec: &mut Vec<T>, salesmen_capacities: &[usize], rng: &mut Option<ChaCha8Rng>) {
    if salesmen_capacities.len() == 1 {
        return;
    }
    let mut si0 = 0;
    let mut si1 = 0;
    let mut i0 = 0;
    let mut i1 = 0;
    while si0 == si1 {
        i0 = crate::utils::rand::chacha_rand_range(0, vec.len(), rng);
        i1 = crate::utils::rand::chacha_rand_range(0, vec.len(), rng);
        si0 = crate::utils::cluster::get_salesman_index(i0, &salesmen_capacities);
        si1 = crate::utils::cluster::get_salesman_index(i1, &salesmen_capacities);
    }
    vec.swap(i0, i1);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_crossover() {
        let order0 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let order1 = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let start = 3;
        let end = 6;
        let child = crossover(&order0, &order1, start, end);
        assert_eq!(child, vec![9, 8, 7, 3, 4, 5, 6, 2, 1, 0]);
    }
}

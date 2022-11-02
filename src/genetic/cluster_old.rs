// use std::fmt;

// use rand::prelude::*;
// use rand_chacha::ChaCha8Rng;
// // https://medium.com/@romanboegli/solving-tsp-with-help-of-genetic-algorithm-in-java-fa2aa4349e8f
// #[derive(Clone)]
// struct Individual<'a, F>
// where
//     F: Fn((f32, f32), (f32, f32)) -> f32 + Clone,
// {
//     points: &'a [(f32, f32)],
//     salesmen_capacities: &'a [usize],
//     length: usize,
//     order: Vec<usize>,
//     fitness: f32,
//     distance_fn: &'a F,
//     seed: Option<u64>,
//     rng: Option<ChaCha8Rng>,
// }

// impl<'a, F> Individual<'a, F>
// where
//     F: Fn((f32, f32), (f32, f32)) -> f32 + Clone,
// {
//     fn new(
//         points: &'a [(f32, f32)],
//         salesmen_capacities: &'a [usize],
//         order: Vec<usize>,
//         distance_fn: &'a F,
//         seed: Option<u64>,
//     ) -> Self {
//         let length = points.len();
//         assert!(length == salesmen_capacities.iter().sum()); // check valid capacities

//         let fitness = -crate::utils::cluster::cluster_metric_from_order(
//             points,
//             salesmen_capacities,
//             &distance_fn,
//             &order,
//         );
//         let rng = seed.map(ChaCha8Rng::seed_from_u64);
//         Self {
//             points,
//             length,
//             distance_fn,
//             fitness,
//             order,
//             salesmen_capacities,
//             seed,
//             rng,
//         }
//     }
//     fn mutate(&mut self) -> Self {
//         let r = crate::utils::rand::chacha_rand_range;
//         let rng = &mut self.rng;
//         let i = r(0, self.length, rng);
//         let j = r(0, self.length, rng);
//         let new_order = {
//             let mut tmp = self.order.clone();
//             tmp.swap(i, j);
//             tmp
//         };
//         let fitness = -crate::utils::cluster::cluster_metric_from_order(
//             self.points,
//             self.salesmen_capacities,
//             &self.distance_fn,
//             &new_order,
//         );
//         let new_rng = self.seed.map(ChaCha8Rng::seed_from_u64);
//         Self {
//             points: self.points,
//             salesmen_capacities: self.salesmen_capacities,
//             length: self.length,
//             order: new_order,
//             distance_fn: self.distance_fn,
//             fitness,
//             seed: self.seed,
//             rng: new_rng,
//         }
//     }
// }

// impl<'a, F> fmt::Display for Individual<'a, F>
// where
//     F: Fn((f32, f32), (f32, f32)) -> f32 + Clone,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Individual {:?}, fitness {}", self.order, self.fitness)
//     }
// }

// pub struct GeneParameters {
//     pub n_individuals: usize,
//     pub max_individuals: usize,
// }

// pub struct GeneticManager<'a, F>
// where
//     F: Fn((f32, f32), (f32, f32)) -> f32 + Clone,
// {
//     individuals: Vec<Individual<'a, F>>,
//     points: &'a [(f32, f32)],
//     salesmen_capacities: &'a [usize],
//     distance_fn: &'a F,
//     seed: Option<u64>,
//     gene_parameters: &'a GeneParameters,
//     rng: Option<ChaCha8Rng>,
// }

// impl<'a, F> GeneticManager<'a, F>
// where
//     F: Fn((f32, f32), (f32, f32)) -> f32 + Clone,
// {
//     pub fn new(
//         points: &'a [(f32, f32)],
//         salesmen_capacities: &'a [usize],
//         distance_fn: &'a F,
//         seed: Option<u64>,
//         gene_parameters: &'a GeneParameters,
//     ) -> Self {
//         let mut rng = seed.map(ChaCha8Rng::seed_from_u64);
//         let mut individuals = vec![];
//         for _ in 0..gene_parameters.n_individuals {
//             let mut order = (0..points.len()).collect::<Vec<usize>>();
//             crate::utils::rand::shuffle_by_chacha_or(&mut order, &mut rng);
//             let individual = Individual::new(points, salesmen_capacities, order, distance_fn, seed);
//             individuals.push(individual);
//         }
//         Self {
//             individuals,
//             points,
//             salesmen_capacities,
//             distance_fn,
//             seed,
//             gene_parameters,
//             rng,
//         }
//     }

//     fn sort_fitness(&mut self) {
//         self.individuals
//             .sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
//     }
//     pub fn step(&mut self) {
//         let mut new_individuals = self.individuals.clone();
//         while new_individuals.len() < self.gene_parameters.max_individuals {
//             let individual = crate::utils::rand::choose_mut(&mut self.individuals, &mut self.rng);
//             new_individuals.push(individual.mutate());
//             // let individual0 = crate::utils::rand::choose_mut(&mut self.individuals, &mut self.rng);
//             // let individual1 = crate::utils::rand::choose_mut(&mut self.individuals, &mut self.rng);
//             // let child = breed(
//             //     individual0,
//             //     individual1,
//             //     self.points,
//             //     self.salesmen_capacities,
//             //     self.distance_fn,
//             //     self.seed,
//             //     &mut self.rng,
//             // );
//             // new_individuals.push(child);
//         }
//         self.individuals = new_individuals;
//         self.sort_fitness();
//         self.individuals
//             .truncate(self.gene_parameters.n_individuals);
//     }

//     pub fn get_fittest(&self) -> &[usize] {
//         &self.individuals[0].order
//     }
// }

// // fn breed<'a, 'b, F>(
// //     individual0: &'a Individual<F>,
// //     individual1: &'a Individual<F>,
// //     points: &'b [(f32, f32)],
// //     salesmen_capacities: &'b [usize],
// //     distance_fn: &'b F,
// //     seed: Option<u64>,
// //     rng: &mut Option<ChaCha8Rng>,
// // ) -> &'b Individual<'a, F>
// // where
// //     F: Fn((f32, f32), (f32, f32)) -> f32 + Clone,
// // {
// //     let length = individual0.order.len();
// //     let start = crate::utils::rand::chacha_rand_range(0, length, rng);
// //     let end = crate::utils::rand::chacha_rand_range(0, length, rng);
// //     let order = crossover(&individual0.order, &individual1.order, start, end);
// //     &Individual::new(points, salesmen_capacities, order, distance_fn, seed)
// // }

// // fn crossover(order0: &[usize], order1: &[usize], start: usize, end: usize) -> Vec<usize> {
// //     let order0 = order0.to_vec();
// //     let order1 = order1.to_vec();
// //     let length = order0.len();
// //     let mut child = vec![length + 1; length]; // use length + 1 as placeholder
// //                                               // fill in genome of first parent
// //     for i in start..end {
// //         child[i] = order0[i];
// //     }
// //     // fill in genome of second parent
// //     let mut j = 0;
// //     for i in 0..length {
// //         if child[i] == length + 1 {
// //             // fillable slot found
// //             let mut l_exit = false;
// //             while !l_exit {
// //                 // check if vertex-index already included
// //                 if child.contains(&order1[j]) {
// //                     j += 1;
// //                 } else {
// //                     l_exit = true;
// //                 }
// //             }
// //             child[i] = order1[j];
// //         }
// //     }
// //     child
// // }

// // impl fmt::Display
// struct Individuals<'a, F>(pub &'a Vec<Individual<'a, F>>)
// where
//     F: Fn((f32, f32), (f32, f32)) -> f32 + Clone;
// impl<'a, F> fmt::Display for Individuals<'a, F>
// where
//     F: Fn((f32, f32), (f32, f32)) -> f32 + Clone,
// {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self.0.iter().fold(Ok(()), |result, x| {
//             result.and_then(|_| writeln!(f, "{}", x))
//         })
//     }
// }
// impl<'a, F> fmt::Display for GeneticManager<'a, F>
// where
//     F: Fn((f32, f32), (f32, f32)) -> f32 + Clone,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", Individuals(&self.individuals))
//     }
// }

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(Debug)]
struct Cluster<F>
where
    F: Fn((f32, f32), (f32, f32)) -> f32 + Clone,
{
    points: Vec<(f32, f32)>,
    length: usize,
    distance_fn: F,
    order: Vec<usize>,
    salesmen_capacities: Vec<usize>,
    rng: Option<ChaCha8Rng>,
}

impl<F> Cluster<F>
where
    F: Fn((f32, f32), (f32, f32)) -> f32 + Clone,
{
    fn new(
        points: &[(f32, f32)],
        salesmen_capacities: &[usize],
        distance_fn: &F,
        seed: Option<u64>,
    ) -> Self {
        let points = points.to_owned();
        let length = points.len();
        assert!(length == salesmen_capacities.iter().sum()); // check valid capacities

        let salesmen_capacities = salesmen_capacities.to_vec();
        let distance_fn = distance_fn.to_owned();
        let order = (0..length).collect::<Vec<usize>>();
        let rng = seed.map(ChaCha8Rng::seed_from_u64);
        Self {
            points,
            length,
            distance_fn,
            order,
            salesmen_capacities,
            rng,
        }
    }

    fn random_pos(&mut self) -> usize {
        match &mut self.rng {
            Some(chacha) => chacha.gen_range(1..(self.points.len())),
            None => rand::thread_rng().gen_range(1..(self.points.len())),
        }
    }

    fn index(&self, i: usize) -> usize {
        (i + self.length) % self.length
    }

    fn access(&self, i: usize) -> (f32, f32) {
        self.points[self.order[self.index(i)]]
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.order.swap(i, j);
    }

    fn delta_distance(&self, i: usize, j: usize) -> f32 {
        let original_order = &self.order;
        let swapped_order;
        {
            let mut vec = self.order.clone();
            vec.swap(i, j);
            swapped_order = vec;
        }
        let original_distance = crate::utils::cluster::cluster_metric_from_order(
            &self.points,
            &self.salesmen_capacities,
            &self.distance_fn,
            original_order,
        );
        let swapped_distance = crate::utils::cluster::cluster_metric_from_order(
            &self.points,
            &self.salesmen_capacities,
            &self.distance_fn,
            &swapped_order,
        );
        swapped_distance - original_distance
    }

    fn change(&mut self, temp: f32) {
        let i = self.random_pos();
        let j = self.random_pos();
        let delta = self.delta_distance(i, j);
        let r: f32 = match &mut self.rng {
            Some(chacha) => chacha.gen(),
            None => rand::thread_rng().gen(),
        };
        if delta < 0.0 || r < (-delta / temp).exp() {
            self.swap(i, j);
        }
    }
}

/// Reorders the points so that nearby points are grouped together.
/// ```
/// use salesman::anneal::cluster::cluster_order;
/// let points = vec![(-0.5, -0.5), (0.5, 0.5), (-0.6, -0.5), (0.5, 0.6)];
/// let salesmen_capacities = vec![2, 2];
/// let distance_fn = salesman::utils::distance::euclidean;
/// let intensity = 10.0;
/// let seed = None;
/// let order = cluster_order(&points, &salesmen_capacities, &distance_fn, intensity, seed);
/// assert!(order[0..2].contains(&0));
/// assert!(order[0..2].contains(&2));
/// assert!(order[2..4].contains(&1));
/// assert!(order[2..4].contains(&3));
/// ```
pub fn cluster_order<F>(
    points: &[(f32, f32)],
    salesmen_capacities: &[usize],
    distance_fn: &F,
    intensity: f32,
    seed: Option<u64>,
) -> Vec<usize>
where
    F: Fn((f32, f32), (f32, f32)) -> f32 + Clone,
{
    let mut cluster = Cluster::new(points, salesmen_capacities, distance_fn, seed);
    if points.len() < 2 {
        return cluster.order;
    }
    let temp_coeff = 1.0 - (-intensity).exp();

    let mut temperature = 100.0 * distance_fn(cluster.access(0), cluster.access(1));
    while temperature > 1e-6 {
        cluster.change(temperature);
        temperature *= temp_coeff;
    }
    cluster.order
}

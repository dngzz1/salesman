use ordered_float::NotNan;
use rand::Rng;

#[derive(Debug, Clone)]
struct Path {
    points: Vec<(f32, f32)>,
    length: usize,
    order: Vec<usize>,
    distances: Vec<f32>,
    salesmen_capacities: Vec<usize>,
}

impl Path {
    fn new(points: &[(f32, f32)], salesmen_capacities: &[usize]) -> Self {
        let points = points.to_owned();
        let length = points.len();
        assert!(length == salesmen_capacities.iter().sum()); // check valid capacities

        let salesmen_capacities = salesmen_capacities.to_vec();
        let order = (0..length).collect::<Vec<usize>>();
        let mut distances = vec![0.0; length * length];
        for i in 0..length {
            for j in 0..length {
                distances[i * length + j] = distance(points[i], points[j]);
            }
        }
        Self {
            points,
            length,
            order,
            distances,
            salesmen_capacities,
        }
    }

    fn random_pos(&self) -> usize {
        rand::thread_rng().gen_range(1..(self.points.len()))
    }

    fn distance(&self, i: usize, j: usize) -> f32 {
        self.distances[self.order[i] * self.length + self.order[j]]
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

    fn delta_distance_slow(&self, i: usize, j: usize) -> f32 {
        let original_order = self.order.clone();
        let swapped_order;
        {
            let mut vec = original_order.clone();
            vec.swap(i, j);
            swapped_order = vec;
        }
        let mut original_distance = 0.0;
        let mut swapped_distance = 0.0;

        for i in 0..self.salesmen_capacities.len() {
            let range_start = self.salesmen_capacities[0..i].iter().sum::<usize>();
            let range_end = self.salesmen_capacities[0..(i + 1)].iter().sum::<usize>() - 1;
            let range = range_start..range_end;
            for j in range {
                original_distance += self.distance(original_order[j], original_order[j + 1]);
                swapped_distance += self.distance(swapped_order[j], swapped_order[j + 1]);
            }
            original_distance +=
                self.distance(original_order[range_end], original_order[range_start]);
            swapped_distance += self.distance(swapped_order[range_end], swapped_order[range_start]);
        }
        swapped_distance - original_distance
    }

    fn change(&mut self, temp: f32) {
        let i = self.random_pos();
        let j = self.random_pos();
        let delta = self.delta_distance_slow(i, j);
        let r: f32 = rand::thread_rng().gen();
        if delta < 0.0 || r < (-delta / temp).exp() {
            self.swap(i, j);
        }
    }
}

fn path_order_once(points: &[(f32, f32)], salesmen_capacities: &[usize]) -> Vec<usize> {
    let mut path = Path::new(points, salesmen_capacities);
    if points.len() < 2 {
        return path.order;
    }
    let intensity = 13.0_f32; // costs more computational time
    let temp_coeff = 1.0 - (-intensity).exp();

    let mut temperature = 100.0 * distance(path.access(0), path.access(1));
    while temperature > 1e-6 {
        path.change(temperature);
        temperature *= temp_coeff;
    }
    path.order
}

fn get_path_from_order(points: &[(f32, f32)], order: &[usize]) -> Vec<(f32, f32)> {
    let mut result = Vec::new();
    for i in 0..order.len() {
        result.push(points[order[i]]);
    }
    result
}

pub fn shortest_path_order(
    points: &[(f32, f32)],
    salesmen_capacities: &[usize],
    num_times: usize,
) -> Vec<usize> {
    let mut loop_distances = Vec::new();
    let mut orders = Vec::new();
    for _ in 0..num_times {
        let order = path_order_once(points, salesmen_capacities);
        orders.push(order.clone());
        let path = get_path_from_order(points, &order);
        loop_distances.push(loop_distance(&path));
    }
    let argmin = argmin(loop_distances);
    orders[argmin].clone()
}

pub fn shortest_path(
    points: &[(f32, f32)],
    salesmen_capacities: &[usize],
    num_times: usize,
) -> Vec<(f32, f32)> {
    let order = shortest_path_order(points, salesmen_capacities, num_times);
    get_path_from_order(points, &order)
}

fn argmin(points: Vec<f32>) -> usize {
    let non_nan_floats: Vec<_> = points
        .iter()
        .cloned()
        .map(NotNan::new) // Attempt to convert each f32 to a NotNan
        .filter_map(Result::ok) // Unwrap the `NotNan`s and filter out the `NaN` values
        .collect();

    let min = non_nan_floats.iter().min().unwrap();
    let index = non_nan_floats
        .iter()
        .position(|element| element == min)
        .unwrap();
    index
}

fn loop_distance(points: &[(f32, f32)]) -> f32 {
    let mut sum = 0.0;
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        sum += distance(points[i], points[j]);
    }
    sum
}

fn distance(p: (f32, f32), q: (f32, f32)) -> f32 {
    let dx = p.0 - q.0;
    let dy = p.1 - q.1;
    (dx * dx + dy * dy).sqrt()
}

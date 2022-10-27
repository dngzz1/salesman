use rand::Rng;

#[derive(Debug, Clone)]
struct Cluster {
    points: Vec<(f32, f32)>,
    length: usize,
    order: Vec<usize>,
    salesmen_capacities: Vec<usize>,
}

impl Cluster {
    fn new(points: &[(f32, f32)], salesmen_capacities: &[usize]) -> Self {
        let points = points.to_owned();
        let length = points.len();
        assert!(length == salesmen_capacities.iter().sum()); // check valid capacities

        let salesmen_capacities = salesmen_capacities.to_vec();
        let order = (0..length).collect::<Vec<usize>>();
        Self {
            points,
            length,
            order,
            salesmen_capacities,
        }
    }

    fn random_pos(&self) -> usize {
        rand::thread_rng().gen_range(0..(self.points.len()))
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
        let original_order = &self.order;
        let swapped_order;
        {
            let mut vec = self.order.clone();
            vec.swap(i, j);
            swapped_order = vec;
        }
        let original_distance =
            cluster_metric_from_order(&self.points, &self.salesmen_capacities, original_order);
        let swapped_distance =
            cluster_metric_from_order(&self.points, &self.salesmen_capacities, &swapped_order);
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

pub fn cluster_order(points: &[(f32, f32)], salesmen_capacities: &[usize]) -> Vec<usize> {
    let mut cluster = Cluster::new(points, salesmen_capacities);
    if points.len() < 2 {
        return cluster.order;
    }
    let intensity = 8.0_f32; // costs more computational time
    let temp_coeff = 1.0 - (-intensity).exp();

    let mut temperature = 100.0 * distance(cluster.access(0), cluster.access(1));
    while temperature > 1e-6 {
        cluster.change(temperature);
        temperature *= temp_coeff;
    }
    cluster.order
}

fn get_cluster_from_order(points: &[(f32, f32)], order: &[usize]) -> Vec<(f32, f32)> {
    let mut result = Vec::new();
    for i in 0..order.len() {
        result.push(points[order[i]]);
    }
    result
}

pub fn best_cluster(points: &[(f32, f32)], salesmen_capacities: &[usize]) -> Vec<(f32, f32)> {
    let order = cluster_order(points, salesmen_capacities);
    get_cluster_from_order(points, &order)
}

fn cluster_i_metric_from_order(
    points: &[(f32, f32)],
    salesmen_capacities: &[usize],
    order: &[usize],
    i: usize,
) -> f32 {
    assert!(i < salesmen_capacities.len());
    let range_start = salesmen_capacities[0..i].iter().sum::<usize>();
    let range_end = salesmen_capacities[0..(i + 1)].iter().sum::<usize>();
    let relevant_points = order[range_start..range_end]
        .iter()
        .map(|&j| points[j])
        .collect::<Vec<_>>();
    cluster_metric(&relevant_points)
}

fn cluster_metric_from_order(
    points: &[(f32, f32)],
    salesmen_capacities: &[usize],
    order: &[usize],
) -> f32 {
    let mut result = 0.0;
    for i in 0..salesmen_capacities.len() {
        result += cluster_i_metric_from_order(points, salesmen_capacities, order, i)
    }
    result
}

fn barycenter(points: &[(f32, f32)]) -> (f32, f32) {
    let n = points.len() as f32;
    let x = points.iter().map(|&point| point.0).sum::<f32>();
    let y = points.iter().map(|&point| point.1).sum::<f32>();
    (x / n, y / n)
}

fn cluster_metric(points: &[(f32, f32)]) -> f32 {
    let b = barycenter(points);
    let mut result = 0.0;
    for &point in points {
        result += distance(point, b);
    }
    result
}

fn distance(p: (f32, f32), q: (f32, f32)) -> f32 {
    let dx = p.0 - q.0;
    let dy = p.1 - q.1;
    (dx * dx + dy * dy).sqrt()
}

#[cfg(test)]
mod test_cluster;

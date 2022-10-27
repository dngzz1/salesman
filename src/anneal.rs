use crate::untangle;

use rand::Rng;

#[derive(Debug, Clone)]
struct Path {
    points: Vec<(f32, f32)>,
    length: usize,
    order: Vec<usize>,
    distances: Vec<f32>,
}

impl Path {
    fn new(points: &[(f32, f32)]) -> Self {
        let points = points.to_owned();
        let length = points.len();
        let order = (0..length).collect::<Vec<usize>>();
        let mut distances = vec![0.0; length * length];
        for i in 0..length {
            for j in 0..length {
                distances[i * length + j] = crate::utils::distance(points[i], points[j]);
            }
        }
        Self {
            points,
            length,
            order,
            distances,
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

    fn delta_distance(&self, i: usize, j: usize) -> f32 {
        let jm1 = self.index(j - 1);
        let jp1 = self.index(j + 1);
        let im1 = self.index(i - 1);
        let ip1 = self.index(i + 1);
        let mut s = self.distance(jm1, i)
            + self.distance(i, jp1)
            + self.distance(im1, j)
            + self.distance(j, ip1)
            - self.distance(im1, i)
            - self.distance(i, ip1)
            - self.distance(jm1, j)
            - self.distance(j, jp1);
        if jm1 == i || jp1 == i {
            s += 2.0 * self.distance(i, j);
        }
        s
    }

    // fn delta_distance2(&self, i: usize, j: usize) -> f32 {
    //     let original_order = self.order.clone();
    //     let swapped_order;
    //     {
    //         let mut vec = original_order.clone();
    //         vec.swap(i, j);
    //         swapped_order = vec;
    //     }
    //     let mut original_distance = 0.0;
    //     let mut swapped_distance = 0.0;
    //     for i in 0..self.length {
    //         let j = (i + 1) % self.length;
    //         original_distance += self.distance(original_order[i], original_order[j]);
    //         swapped_distance += self.distance(swapped_order[i], swapped_order[j]);
    //     }
    //     swapped_distance - original_distance
    // }

    fn change(&mut self, temp: f32) {
        let i = self.random_pos();
        let j = self.random_pos();
        let delta = self.delta_distance(i, j);
        let r: f32 = rand::thread_rng().gen();
        if delta < 0.0 || r < (-delta / temp).exp() {
            self.swap(i, j);
        }
    }
}

fn path_order_once(points: &[(f32, f32)], untangle: bool) -> Vec<usize> {
    let mut path = Path::new(points);
    if points.len() < 2 {
        return path.order;
    }
    let intensity = 10.0_f32; // costs more computational time
    let temp_coeff = 1.0 - (-intensity).exp();

    let mut temperature = 100.0 * crate::utils::distance(path.access(0), path.access(1));
    while temperature > 1e-6 {
        path.change(temperature);
        temperature *= temp_coeff;
    }
    let result;
    if untangle {
        result = untangle::get_untangled_order(points, &path.order);
    } else {
        result = path.order;
    }
    crate::untangle::disconnect_longest_string(&points, &result)
}

pub fn get_path_from_order(points: &[(f32, f32)], order: &[usize]) -> Vec<(f32, f32)> {
    let mut result = Vec::new();
    for i in 0..order.len() {
        result.push(points[order[i]]);
    }
    result
}

pub fn shortest_path_order(points: &[(f32, f32)], num_times: usize) -> Vec<usize> {
    let mut loop_distances = Vec::new();
    let mut orders = Vec::new();
    for _ in 0..num_times {
        let order = path_order_once(points, true);
        orders.push(order.clone());
        let path = get_path_from_order(points, &order);
        loop_distances.push(crate::utils::loop_distance(&path));
    }
    let argmin = crate::utils::argmin(loop_distances);
    orders[argmin].clone()
}

pub fn shortest_path(points: &[(f32, f32)], num_times: usize) -> Vec<(f32, f32)> {
    let order = shortest_path_order(points, num_times);
    get_path_from_order(points, &order)
}
